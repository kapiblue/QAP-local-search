use rand::rngs::ThreadRng;

use crate::candidate_move::CandidateMove;
use crate::qap_problem::QapProblem;
use crate::solution::Solution;
use crate::solvers::solver::Solver;
use crate::{candidate_move, utils::*};
use rand::Rng;
use std::cmp::max;
use std::time::Instant;

pub struct TSSolver<'a> {
    problem: &'a mut QapProblem,
    /// All moves within a neighborhood.
    /// In local search we used the name `candidate moves`,
    /// but in this case candidate moves has a different meaning.
    /// Candidate moves in TS are a subset of neighborhood moves.
    neighborhood_moves: Vec<[usize; 2]>,
    /// A vector of evaluated candidate moves
    candidate_list: Vec<CandidateMove>,
    rng: ThreadRng,
    iter_count: i32,
    /// The number of times a solution is updated
    update_count: i32,
    /// The tabu list is stored in a vector of vectors
    tabu_list: Vec<Vec<i32>>,
    /// The number of iterations until deactivation
    tenure: i32,
    /// The range of delta values in the current elite population
    delta_range: i32,
    /// The initial solution
    initial_solution: Option<Solution>,
    // The fraction of the neighborhood to check
    cn_ratio: f32,
    // candidate list size
    k: usize,
    // Stopping criterion
    lack_improvement_iter: i32,
}

fn create_tabu_list(n: usize) -> Vec<Vec<i32>> {
    let mut tabu_list: Vec<Vec<i32>> = vec![];
    for i in 0..n - 1 {
        tabu_list.push(vec![0; n - i - 1])
    }
    return tabu_list;
}

impl<'a> TSSolver<'a> {
    /// Constructor
    pub fn new(
        problem: &'a mut QapProblem,
        tenure: i32,
        cn_ratio: f32,
        k: usize,
        lack_improvement_iter: i32,
    ) -> TSSolver<'a> {
        // Vector of pairs (i, j), moves in order
        let neighborhood_moves = generate_pairs(problem.get_n());
        let candidate_list = vec![];
        let rng = rand::thread_rng();

        let iter_count: i32 = 0;
        let update_count: i32 = 0;

        let default_time_limit: u128 = 1000;

        let tabu_list: Vec<Vec<i32>> = create_tabu_list(problem.get_n());
        let delta_range = 0;

        let initial_solution = None;

        TSSolver {
            problem,
            neighborhood_moves,
            candidate_list,
            rng,
            iter_count,
            update_count,
            tabu_list,
            tenure,
            delta_range,
            initial_solution,
            cn_ratio,
            k,
            lack_improvement_iter,
        }
    }

    /// Constructs candidate list by evaluating a subset of moves from the neighborhood
    /// and selecting k of them. The number of evaluated moves is equal to the
    /// size of the neighborhood times the cn_ratio (a float between 0 and 1).
    fn construct_elite_candidate_list(&mut self, current_solution: &Solution) -> () {
        // Remove any remaining moves from the list
        self.candidate_list.clear();
        // Calculate how many nieghborhood moves should be checked
        let list_size = (self.neighborhood_moves.len() as f32 * self.cn_ratio) as usize;
        // Initialize loop counter
        let mut i: usize = 0;
        // Randomize the order of neighboring moves
        // permute_array(&mut self.rng, &mut self.neighborhood_moves);
        while i < list_size {
            let pair = &self.neighborhood_moves[i];
            let delta: i32 = current_solution.calculate_delta(
                self.problem.matrix_a_ref(),
                self.problem.matrix_b_ref(),
                pair,
            );
            // Create a move from the pair and delta
            let candidate_move = CandidateMove::new(*pair, delta);
            // Push the move to the list
            self.candidate_list.push(candidate_move);
            i += 1;
        }
        // Sort the candidate moves by descending delta
        self.candidate_list.sort_by_key(|c| c.get_delta());
        self.delta_range = 0;
        // Select k best moves
        self.candidate_list.truncate(self.k);
        // Reverse the vector to be able to pop best moves
        self.candidate_list.reverse();
        self.calculate_delta_range();
    }

    /// Recalculates delta for the candidate moves
    /// after and exchange
    fn recalculate_candidate_list_delta(&mut self, current_solution: &Solution) -> () {
        for candidate_move in self.candidate_list.iter_mut() {
            candidate_move.delta = current_solution.calculate_delta(
                self.problem.matrix_a_ref(),
                self.problem.matrix_b_ref(),
                &candidate_move.pair,
            );
        }
        // Sort by descending delta
        self.candidate_list.sort_by_key(|c| -c.get_delta());
    }
    /// Calculates the range of delta values in the candidate list.
    /// Assumes the vector is sorted in descending order.
    fn calculate_delta_range(&mut self) -> () {
        self.delta_range =
            self.candidate_list[0].delta - self.candidate_list[self.candidate_list.len() - 1].delta
    }

    fn is_good_quality(&self, can_move: &CandidateMove) -> bool {
        can_move.delta < self.delta_range
    }

    /// Selects the best candidate move in the list
    /// and according to the aspiration criterion
    fn select_best_move(&mut self, current_solution: &Solution) -> CandidateMove {
        let mut best_candidate_move = CandidateMove::new([1, 1], 1000);
        // Whether we need to regenerate the candidate list
        let mut is_regeneration_needed = false;
        if self.candidate_list.len() == 0 {
            is_regeneration_needed = true;
        }
        // Whether the move was found
        let mut is_move_not_found = true;
        while is_move_not_found {
            if is_regeneration_needed {
                // Generate the candidate list
                self.construct_elite_candidate_list(&current_solution);
            }
            let can_move = self.candidate_list.pop().unwrap();
            // Checking the aspiration criteria
            // Always accept an improving move
            if can_move.delta <= 0 {
                is_move_not_found = false;
                best_candidate_move = can_move;
            }
            // Check the tabu list
            else {
                let i = can_move.pair[0];
                let j = can_move.pair[1] - i - 1;
                if self.tabu_list[i][j] == 0 {
                    // Check move quality
                    if self.is_good_quality(&can_move) {
                        is_move_not_found = false;
                        best_candidate_move = can_move;
                    } else {
                        is_regeneration_needed = true;
                    }
                }
            }
            if self.candidate_list.len() == 0 {
                is_regeneration_needed = true
            }
        }
        // Due to the aspiration criterion, we accept any improving move
        // If the move is not improving, we will check the tabu list
        best_candidate_move
    }

    /// Updates the tabu list. It takes advantage of the fact that in the move (pair (i,j)), i < j
    fn update_tabu_list(&mut self, selected_move: &CandidateMove) -> () {
        let selected_i = selected_move.pair[0];
        let selected_j = selected_move.pair[1] - selected_i - 1;
        for i in 0..self.tabu_list.len() {
            for j in 0..self.tabu_list[i].len() {
                // Set tenure value for the selected move
                if i == selected_i && j == selected_j {
                    self.tabu_list[i][j] = self.tenure
                // Or update the field if it is not selected
                } else {
                    self.tabu_list[i][j] = max(0, self.tabu_list[i][j] - 1);
                }
            }
        }
        // println!("Tabu list: {:?}", self.tabu_list);
    }

    pub fn solve_tabu_search(&mut self, mut initial_solution: Solution) -> Solution {
        initial_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        self.initial_solution = Some(Solution::new(initial_solution.get_solution_array()));
        self.initial_solution
            .as_mut()
            .unwrap()
            .set_eval(initial_solution.get_eval());
        // Assign current colution to initial solution
        let mut current_solution = initial_solution.clone();
        // Assign best solution to initial solution
        let mut best_solution = initial_solution.clone();

        // Initialize the counter of iterations without an improvement
        let mut lack_improvement_iter = 0;

        while lack_improvement_iter < self.lack_improvement_iter {
            let selected_move = self.select_best_move(&current_solution);
            // Apply the move
            current_solution.exchange_n_facilities(&vec![selected_move.pair]);
            self.update_tabu_list(&selected_move);
            self.recalculate_candidate_list_delta(&current_solution);

            // Update the evaluation of the current solution
            current_solution.set_eval(current_solution.get_eval() + selected_move.delta);

            // Update the best solution if a better one was found
            if current_solution.get_eval() <= best_solution.get_eval() {
                best_solution = current_solution.clone();
                lack_improvement_iter = 0;
                self.update_count += 1;
            } else {
                lack_improvement_iter += 1;
            }
            self.iter_count += 1;
        }
        best_solution
    }
}

impl<'a> Solver for TSSolver<'a> {
    fn solve(&mut self) -> Solution {
        // Start from a random solution
        let initial_solution: Solution = self.problem.generate_random_solution();
        self.solve_tabu_search(initial_solution)
    }
    fn get_iter_count(&self) -> i32 {
        self.iter_count
    }
    fn get_update_count(&self) -> i32 {
        self.update_count
    }
    fn get_initial_solution(&self) -> Option<Solution> {
        self.initial_solution.clone()
    }
}

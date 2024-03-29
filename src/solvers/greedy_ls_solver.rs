use rand::rngs::ThreadRng;

use crate::qap_problem::QapProblem;
use crate::solution::Solution;
use crate::solvers::solver::Solver;
use crate::utils::*;

pub struct GreedyLSSolver<'a> {
    problem: &'a QapProblem,
    candidate_moves: Vec<[usize; 2]>,
    rng: ThreadRng,
    iter_count: i32,   // The number of times the LS loop is ran
    update_count: i32, // The number of times a solution is updated
    initial_solution: Option<Solution>,
}

impl<'a> GreedyLSSolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem) -> GreedyLSSolver<'a> {
        // Vector of pairs (i, j), moves in order
        let candidate_moves = generate_pairs(problem.get_n());
        let rng = rand::thread_rng();
        let iter_count: i32 = 0;
        let update_count: i32 = 0;
        let initial_solution = None;
        GreedyLSSolver {
            problem,
            candidate_moves,
            rng,
            iter_count,
            update_count,
            initial_solution,
        }
    }

    fn generate_random_solution(&mut self) -> Solution {
        let n = self.problem.get_n();
        let mut solution_array = vec![0; n];
        arange(&mut solution_array, 0, 1);
        permute_array(&mut self.rng, &mut solution_array);
        Solution::new(solution_array)
    }

    fn solve_greedy(&mut self, initial_solution: Solution) -> Solution {
        let mut initial_solution = initial_solution;
        initial_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        self.initial_solution = Some(Solution::new(initial_solution.get_solution_array()));
        self.initial_solution.as_mut().unwrap().set_eval(initial_solution.get_eval());
        
        let mut current_solution = initial_solution;
        // println!("LS initial solution: {}", current_solution);
        // Randomize the order of pairs
        permute_array(&mut self.rng, &mut self.candidate_moves);
        let mut iter_count = 0;
        self.update_count = 0;
        let mut i = 0;
        loop {
            let pair = self.candidate_moves[i];
            let delta = current_solution.calculate_delta(
                self.problem.matrix_a_ref(),
                self.problem.matrix_b_ref(),
                &pair,
            );
            if delta < 0 {
                current_solution.exchange_facilities(&pair);
                permute_array(&mut self.rng, &mut self.candidate_moves);
                i = 0;
                self.update_count = self.update_count + 1;
            }
            if i == self.candidate_moves.len() - 1 {
                break;
            }
            i += 1;
            // Update the iteration count
            iter_count = iter_count + 1;
        }
        self.iter_count = iter_count;
        current_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        return current_solution;
    }
}

impl<'a> Solver for GreedyLSSolver<'a> {
    // Just greedy
    fn solve(&mut self) -> Solution {
        let initial_solution = self.generate_random_solution();
        self.solve_greedy(initial_solution)
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

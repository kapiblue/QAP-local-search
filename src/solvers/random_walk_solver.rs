use rand::rngs::ThreadRng;

use crate::qap_problem::QapProblem;
use crate::solution::Solution;
use crate::solvers::solver::Solver;
use crate::utils::*;
use std::time::Instant;
use rand::Rng;

pub struct RandomWalkSolver<'a> {
    problem: &'a QapProblem,
    candidate_moves: Vec<[usize; 2]>,
    rng: ThreadRng,
    iter_count: i32,   // The number of times the LS loop is ran
    update_count: i32, // The number of times a solution is updated
    time_limit: u128,
}

impl<'a> RandomWalkSolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem, time_limit: Option<u128>) -> RandomWalkSolver<'a> {
        // Vector of pairs (i, j), moves in order
        let candidate_moves = generate_pairs(problem.get_n());
        let rng = rand::thread_rng();
        let iter_count: i32 = 0;
        let update_count: i32 = 0;
        let default_time_limit: u128 = 1000;
        let time_limit: u128 = time_limit.unwrap_or(default_time_limit);

        RandomWalkSolver {
            problem,
            candidate_moves,
            rng,
            iter_count,
            update_count,
            time_limit,
        }
    }

    fn generate_random_solution(&mut self) -> Solution {
        let n = self.problem.get_n();
        let mut solution_array = vec![0; n];
        arange(&mut solution_array, 0, 1);
        permute_array(&mut self.rng, &mut solution_array);
        Solution::new(solution_array)
    }

    fn solve_random_walk(&mut self, initial_solution: Solution) -> Solution {
        let mut current_solution = initial_solution;
        println!("Random walk initial solution: {}", current_solution);

        current_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        let mut current_score = current_solution.get_eval();

        let mut best_score = current_score;
        let mut current_array: Vec<usize> = current_solution.get_solution_array();
        let mut best_solution = Solution::new(current_array);
        // The way how we randomize the random moves is more or less
        // how I recall it from the labs discussion. We permute the array
        // with candidates only ones, select a random point where we start
        // and we iterate over pairs in a form of a cycle.
        // Randomize the order of pairs
        permute_array(&mut self.rng, &mut self.candidate_moves);
        let range: i32 = self.candidate_moves.len() as i32;
        let mut move_pointer: usize = self.rng.gen_range(0..range) as usize;
        let start = Instant::now();
        let mut elapsed: u128 = 0;
        while elapsed < self.time_limit {
            elapsed = start.elapsed().as_millis();
            // perform a random move
            let pair = self.candidate_moves[move_pointer];
            // increase a move pointer to select different move in
            // the next iteration
            move_pointer += 1;

            let delta: i32 = current_solution.calculate_delta(
                self.problem.matrix_a_ref(),
                self.problem.matrix_b_ref(),
                &pair,
            );

            // we perform a random move always
            current_solution.exchange_facilities(&pair);
            current_score = current_score + delta;

            if current_score < best_score {
                // if the performed move cause the improvement
                // overwrite the best solutions
                current_array = current_solution.get_solution_array();
                best_solution = Solution::new(current_array);
                best_score = current_score;
                
            }
            if move_pointer == self.candidate_moves.len() {
                move_pointer = 0;
            }
        }
        best_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        best_solution
    }

    
}

impl<'a> Solver for RandomWalkSolver<'a> {
    // Just greedy
    fn solve(&mut self) -> Solution {
        let initial_solution = self.generate_random_solution();
        let solution = self.solve_random_walk(initial_solution);
        return solution;
        // TODO: move generate random solution to QAP problem class
    }
    fn get_iter_count(&self) -> i32 {
        0
    }
    fn get_update_count(&self) -> i32 {
        0
    }
}

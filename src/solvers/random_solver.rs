use std::thread::current;
use std::time::Instant;

use rand::rngs::ThreadRng;

use crate::qap_problem::QapProblem;
use crate::solvers::solver::Solver;
use crate::solution::Solution;
use crate::utils::*;

/// Random Solver stores a ThreadRng object to reuse
pub struct RandomSolver<'a> {
    problem: &'a QapProblem,
    rng: ThreadRng,
    time_limit: u128,
    iter_count: i32,
}

impl<'a> RandomSolver<'a> {
    /// Constructor
    pub fn new(problem: &'a QapProblem, time_limit: Option<u128>) -> RandomSolver<'a> {
        let rng = rand::thread_rng();
        let default_time_limit: u128 = 1000;
        let time_limit: u128 = time_limit.unwrap_or(default_time_limit);
        let iter_count: i32 = 0;
        RandomSolver {
            problem,
            rng,
            time_limit,
            iter_count
        }
    }

    
    // Private method specific to RandomSolver
    fn generate_random_solution(&mut self) -> Solution {
        let n = self.problem.get_n();
        let mut solution_array = vec![0; n];
        arange(&mut solution_array, 0, 1);
        permute_array(&mut self.rng, &mut solution_array);
        Solution::new(solution_array)
    }
    // pub fn solve(&mut self) -> Solution {
    //     self.generate_random_solution()
    // }
}


impl<'a> Solver for RandomSolver<'a>{
    /// Generates random solutions and returns the best solution
    /// found before the time limit
    fn solve(&mut self) -> Solution {
        let mut solution =  self.generate_random_solution();
        
        let mut current_array: Vec<usize> = solution.get_solution_array();

        let mut best_solution = Solution::new(current_array);
        let mut best_score = solution.evaluate(
            self.problem.matrix_a_ref(), 
            self.problem.matrix_b_ref()
        );

        let start = Instant::now();
        let mut elapsed: u128 = 0;
        let mut iter_count = 0;
        while elapsed < self.time_limit {
            elapsed = start.elapsed().as_millis();

            // generate a new totally random solution
            solution = self.generate_random_solution();
            let score = solution.evaluate(
                self.problem.matrix_a_ref(), 
                self.problem.matrix_b_ref()
            );

            if score < best_score {
                best_score = score;
                current_array = solution.get_solution_array();
                best_solution = Solution::new(current_array);
            }
            iter_count = iter_count + 1;
        }

        best_solution.evaluate(
            self.problem.matrix_a_ref(), 
            self.problem.matrix_b_ref()
        );
        self.iter_count = iter_count;
        best_solution
    }
    fn get_iter_count(&self) -> i32 {
        self.iter_count
    }
    fn get_update_count(&self) -> i32 {
        0
    }
    fn get_initial_solution(&self) -> Option<Solution> {
        None
    }
}
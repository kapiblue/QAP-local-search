use std::thread::current;
use std::time::Instant;

use rand::rngs::ThreadRng;

use crate::qap_problem::QapProblem;
use crate::solvers::solver::Solver;
use crate::solution::Solution;
use crate::utils::*;

pub struct RandomSolver<'a> {
    problem: &'a QapProblem,
    rng: ThreadRng,
    time_limit: u128,
}

impl<'a> RandomSolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem, time_limit: Option<u128>) -> RandomSolver<'a> {
        let rng = rand::thread_rng();
        let default_time_limit: u128 = 1000;
        let time_limit: u128 = time_limit.unwrap_or(default_time_limit);
        RandomSolver {
            problem,
            rng,
            time_limit 
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
        }

        best_solution.evaluate(
            self.problem.matrix_a_ref(), 
            self.problem.matrix_b_ref()
        );
        best_solution
    }
    fn get_iter_count(&self) -> i32 {
        0
    }
    fn get_update_count(&self) -> i32 {
        0
    }
}
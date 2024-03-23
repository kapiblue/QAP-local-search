use rand::rngs::ThreadRng;

use crate::qap_problem::QapProblem;
use crate::solution::Solution;
use crate::solvers::solver::Solver;
use crate::utils::*;

pub struct SteepestLSSolver<'a> {
    problem: &'a QapProblem,
    candidate_moves: Vec<[usize; 2]>,
    rng: ThreadRng,
}

impl<'a> SteepestLSSolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem) -> SteepestLSSolver<'a> {
        // Vector of pairs (i, j), moves in order
        let candidate_moves = generate_pairs(problem.get_n());
        let rng = rand::thread_rng();
        SteepestLSSolver {
            problem,
            candidate_moves,
            rng,
        }
    }

    fn generate_random_solution(&mut self) -> Solution {
        let n = self.problem.get_n();
        let mut solution_array = vec![0; n];
        arange(&mut solution_array, 0, 1);
        permute_array(&mut self.rng, &mut solution_array);
        Solution::new(solution_array)
    }

    fn solve_steepest(&mut self, initial_solution: Solution) -> Solution {
        let mut current_solution = initial_solution;
        // println!("Current solution: {}", current_solution);
        let mut i = 1;
        loop {
            let mut best_delta: i32 = num_traits::zero();
            let mut best_pair: [usize; 2] = [0,0];
    
            for &pair in &self.candidate_moves {
                let delta: i32 = current_solution.calculate_delta(
                    self.problem.matrix_a_ref(),
                    self.problem.matrix_b_ref(),
                    &pair,
                );
    
                // println!("Delta for pair {:?}: {}", pair, delta);
    
                if delta < best_delta {
                    best_delta = delta;
                    best_pair = pair;
                }
            }
            
            if best_delta < num_traits::zero() {
                current_solution.exchange_facilities(&best_pair);
                // println!("Best pair: {:?}, Delta: {}; at epoch {}", best_pair, best_delta, i);
            } else {
                break;
            }
            i = i + 1;
        }
    
        current_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        current_solution
    }
}


impl<'a> Solver for SteepestLSSolver<'a> {
    // Just greedy
    fn solve(&mut self) -> Solution {
        let initial_solution: Solution = self.generate_random_solution();
        self.solve_steepest(initial_solution)
        // TODO: move generate random solution to QAP problem class
        
    }
    fn get_iter_count(&self) -> i32 {
        0
    }
    fn get_update_count(&self) -> i32 {
        0
    }
}
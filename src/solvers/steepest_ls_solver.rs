use rand::rngs::ThreadRng;

use crate::qap_problem::QapProblem;
use crate::solution::Solution;
use crate::solvers::solver::Solver;
use crate::utils::*;

pub struct SteepestLSSolver<'a> {
    problem: &'a QapProblem,
    candidate_moves: Vec<[usize; 2]>,
    two_candidates_moves: Vec<Vec<[usize; 2]>>,
    rng: ThreadRng,
    iter_count: i32,   // The number of times the LS loop is ran
    update_count: i32, // The number of times a solution is updated
    initial_solution: Option<Solution>,
}

impl<'a> SteepestLSSolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem) -> SteepestLSSolver<'a> {
        // Vector of pairs (i, j), moves in order
        let candidate_moves = generate_pairs(problem.get_n());
        let two_candidates_moves = generate_two_pairs(problem.get_n());
        let rng = rand::thread_rng();
        let iter_count: i32 = 0;
        let update_count: i32 = 0;
        let initial_solution = None;
        SteepestLSSolver {
            problem,
            candidate_moves,
            two_candidates_moves,
            rng,
            iter_count,
            update_count,
            initial_solution
        }
    }

    fn generate_random_solution(&mut self) -> Solution {
        let n = self.problem.get_n();
        let mut solution_array = vec![0; n];
        arange(&mut solution_array, 0, 1);
        permute_array(&mut self.rng, &mut solution_array);
        Solution::new(solution_array)
    }

    fn solve_steepest(&mut self, mut initial_solution: Solution) -> Solution {
        initial_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        self.initial_solution = Some(Solution::new(initial_solution.get_solution_array()));
        self.initial_solution.as_mut().unwrap().set_eval(initial_solution.get_eval());
        let mut current_solution = initial_solution;
        // println!("Current solution: {}", current_solution);

        let num_candidates = self.candidate_moves.len();

        // let mut i = 1;
        let mut iter_count = 0;
        self.update_count = 0;
        loop {
            let mut best_delta: i32 = num_traits::zero();
            // let mut best_pair: [usize; 2] = [0,0];

            let mut best_pairs  = vec![];
    
            for &pair in &self.candidate_moves {
                let delta: i32 = current_solution.calculate_delta(
                    self.problem.matrix_a_ref(),
                    self.problem.matrix_b_ref(),
                    &pair,
                );
    
                // println!("Delta for pair {:?}: {}", pair, delta);
    
                if delta < best_delta {
                    best_delta = delta;
                    best_pairs = vec![pair];
                }
            }

            // exchange 2 pairs in the solution
            // for i in 0..num_candidates{
            //     for j in i+1..num_candidates{
            //         let pair1 = self.candidate_moves[i];
            //         let pair2 = self.candidate_moves[j];
            //         best_pairs = vec![pair1, pair2];
            //         let delta: i32 = current_solution.calculate_n_deltas(
            //             self.problem.matrix_a_ref(), 
            //             self.problem.matrix_b_ref(),
            //             &best_pairs);
            //         if delta < best_delta {
            //             best_delta = delta;
            //             best_pairs = vec![pair1, pair2];
            //         }
            //     }
            // }
            // for pairs in &self.two_candidates_moves {
            //     let delta: i32 = current_solution.calculate_n_deltas(
            //         self.problem.matrix_a_ref(),
            //         self.problem.matrix_b_ref(),
            //         &pairs,
            //     );
            //     if delta < best_delta {
            //         best_delta = delta;
            //         best_pairs = pairs.to_vec();
            //     }
            // }
            
            if best_delta < num_traits::zero() {
                current_solution.exchange_n_facilities(&best_pairs);
                self.update_count = self.update_count + 1;
                // println!("Best pair: {:?}, Delta: {}; at epoch {}", best_pairs, best_delta, iter_count);
            } else {
                break;
            }
            // i = i + 1;
            iter_count = iter_count + 1;
        }
        self.iter_count = iter_count;
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
        self.iter_count
    }
    fn get_update_count(&self) -> i32 {
        self.update_count
    }
    fn get_initial_solution(&self) -> Option<Solution> {
        self.initial_solution.clone()
    }
}
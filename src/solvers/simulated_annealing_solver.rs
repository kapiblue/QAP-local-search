use rand::rngs::ThreadRng;

use crate::qap_problem::QapProblem;
use crate::solution::Solution;
use crate::solvers::solver::Solver;
use crate::utils::*;
use std::time::Instant;
use rand::Rng;

pub struct SASolver<'a> {
    problem: &'a QapProblem,
    candidate_moves: Vec<[usize; 2]>,
    rng: ThreadRng,
    iter_count: i32,
    time_limit: u128,
    update_count: i32, // The number of times a solution is updated
    temperature: f32,
}

impl<'a> SASolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem, time_limit: Option<u128>) -> SASolver<'a> {
        // Vector of pairs (i, j), moves in order
        let candidate_moves = generate_pairs(problem.get_n());
        let rng = rand::thread_rng();
        let iter_count: i32 = 0;
        let update_count: i32 = 0;
        let default_time_limit: u128 = 1000;
        let time_limit: u128 = time_limit.unwrap_or(default_time_limit);
        let temperature: f32 = 0.95;

        SASolver {
            problem,
            candidate_moves,
            rng,
            iter_count,
            time_limit,
            update_count,
            temperature,
        }
    }

    fn solve_simulated_annealing(&mut self, initial_solution: Solution) -> Solution {
        let mut p = 0; // holds the iterations without improvement
        let mut q = 0; // holds the acceptance probability
        let mut k = 0; // holds the number of iterations through L
        let mut current_solution = initial_solution;
        current_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        // let l be a half of the number of candidate moves, rounded down
        let l = self.candidate_moves.len() / 2;
        // draw a random number that indicates which candidate we take
        let mut j = self.rng.gen_range(0..self.candidate_moves.len());

        let mut current_score = current_solution.get_eval();

        // max number of iterations without improvement
        let max_iter = l * 10;

        loop {
            for i in 0..l {
                let pair = self.candidate_moves[j];
                let delta = current_solution.calculate_delta(
                    self.problem.matrix_a_ref(),
                    self.problem.matrix_b_ref(),
                    &pair
                );
                if delta < 0 {
                    current_solution.exchange_facilities(&pair);
                    current_score += delta;
                    self.update_count += 1;
                    p = 0;
                } else {
                    p += 1;
                    let q = (-delta as f32 / self.temperature).exp();
                    let r = self.rng.gen_range(0.0..1.0);
                    if q > r {
                        current_solution.exchange_facilities(&pair);
                        current_score += delta;
                        self.update_count += 1;
                    }
                }
                j = (j + 1) % self.candidate_moves.len();
            }
            k += 1;
            self.temperature *= 0.90;

            if self.temperature < 0.01 {
                break;
            }
            if p >= max_iter {
                break;
            }
        }
        current_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        current_solution
    }

    
}

impl<'a> Solver for SASolver<'a> {
    // Just greedy
    fn solve(&mut self) -> Solution {
        let initial_solution: Solution = self.generate_random_solution();
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

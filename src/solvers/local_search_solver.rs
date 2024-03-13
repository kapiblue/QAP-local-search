use rand::rngs::ThreadRng;

use crate::qap_problem::QapProblem;
use crate::solution::Solution;
use crate::solvers::solver::Solver;
use crate::utils::*;

pub struct LocalSearchSolver<'a> {
    problem: &'a QapProblem,
    candidate_moves: Vec<[usize; 2]>,
    rng: ThreadRng,
}

impl<'a> LocalSearchSolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem) -> LocalSearchSolver<'a> {
        // Vector of pairs (i, j), moves in order
        let candidate_moves = generate_pairs(problem.get_n());
        let rng = rand::thread_rng();
        LocalSearchSolver {
            problem,
            candidate_moves,
            rng,
        }
    }
    // Just greedy
    pub fn solve(&mut self, initial_solution: Solution) -> Solution {
        // Randimize the order of pairs
        let mut current_solution = initial_solution;
        println!("Current solution: {}", current_solution);
        permute_array(&mut self.rng, &mut self.candidate_moves);
        let mut i = 0;
        loop {
            let pair = self.candidate_moves[i];
            let delta = current_solution.calculate_delta(
                self.problem.matrix_a_ref(),
                self.problem.matrix_b_ref(),
                &pair,
            );
            println!("Delta: {}", delta);
            if delta < 0 {
                current_solution.exchange_facilities(&pair);
                permute_array(&mut self.rng, &mut self.candidate_moves);
                i = 0;
            }
            if i == self.candidate_moves.len()-1 {
                break;
            }
            i += 1;
        }
        current_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        return current_solution;
    }
}

use rand::rngs::ThreadRng;

use crate::qap_problem::QapProblem;
use crate::solution::Solution;
use crate::solvers::solver::Solver;
use crate::utils::*;
use rand::Rng;

pub struct SASolver<'a> {
    problem: &'a QapProblem,
    candidate_moves: Vec<[usize; 2]>,
    rng: ThreadRng,
    iter_count: i32,
    update_count: i32, // The number of times a solution is updated
    temperature: f32,
    /// The initial solution
    initial_solution: Option<Solution>,

    temp_mul: f32,
    iter_mul: usize,
    l_div: i32,
}

impl<'a> SASolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem, temp_mul: f32, iter_mul: usize, l_div: i32) -> SASolver<'a> {
        // Vector of pairs (i, j), moves in order
        let candidate_moves = generate_pairs(problem.get_n());
        let rng = rand::thread_rng();
        let iter_count: i32 = 0;
        let temperature: f32 = 0.95;
        let update_count: i32 = 0;
        let initial_solution = None;

        let temp_mul = temp_mul;
        let iter_mul = iter_mul;
        let l_div = l_div;

        SASolver {
            problem,
            candidate_moves,
            rng,
            iter_count,
            update_count,
            temperature,
            initial_solution,
            temp_mul,
            iter_mul,
            l_div,
        }
    }

    fn generate_random_solution(&mut self) -> Solution {
        let n = self.problem.get_n();
        let mut solution_array = vec![0; n];
        arange(&mut solution_array, 0, 1);
        permute_array(&mut self.rng, &mut solution_array);
        Solution::new(solution_array)
    }

    pub fn compute_initial_temperature(&mut self) {
        // generate random 1,000 solutions
        // for each solution generate random neighbor and compute delta
        // compute average delta

        let mut deltas = Vec::new();
        for _ in 0..1000 {
            let mut solution = self.generate_random_solution();
            solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
            let pair = self.candidate_moves[self.rng.gen_range(0..self.candidate_moves.len())];
            let delta = solution.calculate_delta(
                self.problem.matrix_a_ref(),
                self.problem.matrix_b_ref(),
                &pair
            );
            deltas.push(delta.abs());
        }
        let sum: i128 = deltas.iter().map(|&delta| delta as i128).sum();
        let avg = sum as f32 / deltas.len() as f32;

        // target initial probability = 0.95
        let prob: f32 = 0.95;
        // take ln from prob
        let ln_prob: f32 = prob.ln();
        // compute initial temperature
        let temperature: f32 = -avg / ln_prob;
        self.temperature = temperature;
    }

    fn solve_simulated_annealing(&mut self, mut initial_solution: Solution) -> Solution {
        let mut p = 0; // holds the iterations without improvement
        // let mut k = 0; // holds the number of iterations through L
        initial_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        self.initial_solution = Some(Solution::new(initial_solution.get_solution_array()));
        self.initial_solution
            .as_mut()
            .unwrap()
            .set_eval(initial_solution.get_eval());
        let mut current_solution = initial_solution.clone();
        // current_solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        // let l be a quarter of the number of candidate moves, rounded down
        let l = self.candidate_moves.len() / self.l_div as usize;
        // draw a random number that indicates which candidate we take
        let mut j = self.rng.gen_range(0..self.candidate_moves.len());

        let mut local_temp = self.temperature;
        // let mut current_score = current_solution.get_eval();

        // max number of iterations without improvement
        let max_iter = l * self.iter_mul;

        self.iter_count = 0;

        loop {
            for _ in 0..l {
                let pair = self.candidate_moves[j];
                let delta = current_solution.calculate_delta(
                    self.problem.matrix_a_ref(),
                    self.problem.matrix_b_ref(),
                    &pair
                );
                if delta < 0 {
                    current_solution.exchange_facilities(&pair);
                    // current_score += delta;
                    self.update_count += 1;
                    p = 0;
                } else {
                    p += 1;
                    let q = (-delta as f32 / local_temp).exp();
                    let r = self.rng.gen_range(0.0..1.0);
                    if q > r {
                        current_solution.exchange_facilities(&pair);
                        self.update_count += 1;
                        // current_score += delta;
                    }
                }
                j = (j + 1) % self.candidate_moves.len();
                self.iter_count += 1;

                // print status every 10000 iterations
                // if self.iter_count % 10000 == 0 {
                //     println!(
                //         "Iteration: {}; Best found solution evaluation: {}; temp: {}; original temp: {}",
                //         self.iter_count,
                //         current_score,
                //         local_temp,
                //         self.temperature
                //     );
                // }
            }
            

            // k += 1;
            local_temp *= self.temp_mul;

            if local_temp < 0.01 && p >= max_iter{
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
        self.solve_simulated_annealing(initial_solution)
    }
    fn get_iter_count(&self) -> i32 {
        self.iter_count
    }
    fn get_update_count(&self) -> i32 {
        0
    }
    fn get_initial_solution(&self) -> Option<Solution> {
        self.initial_solution.clone()
    }
}

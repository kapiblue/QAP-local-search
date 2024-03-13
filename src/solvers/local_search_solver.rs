
use crate::qap_problem::QapProblem;
use crate::solvers::solver::Solver;
use crate::solution::Solution;
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
        let pairs = generate_pairs(problem.get_n());
        let rng = rand::thread_rng();
        LocalSearchSolver { problem, pairs, rng };

    }

    // Private method specific to RandomSolver
    fn generate_random_solution(&mut self) -> Solution {
        let n = self.problem.get_n();
        let mut solution_array = vec![0; n];
        arange(&mut solution_array, 1, 1);
        permute_array(&mut self.rng, &mut solution_array);
        Solution::new(solution_array)
    }
    pub fn solve(&mut self, initial_solution: Solution) -> Solution {
        // Randimize the order of pairs
        permute_array(&mut self.rng, &mut candidate_moves);

        self.generate_random_solution()
    }
}


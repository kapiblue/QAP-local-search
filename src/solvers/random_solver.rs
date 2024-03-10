use crate::qap_problem::QapProblem;
use crate::solvers::solver::Solver;
use crate::solution::Solution;
use crate::utils::*;

pub struct RandomSolver<'a> {
    problem: &'a QapProblem,
}

impl<'a> RandomSolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem) -> RandomSolver<'a> {
        RandomSolver { problem }
    }

    // Private method specific to RandomSolver
    fn generate_random_solution(&self) -> Solution {
        let mut rng = rand::thread_rng();
        let n = self.problem.get_n();
        let mut solution_array = vec![0; n];
        arange(&mut solution_array, 1, 1);
        permute_array(&mut rng, &mut solution_array);
        Solution::new(solution_array)
    }
}


impl<'a> Solver for RandomSolver<'a>{
    fn solve(&self) -> Solution {
        self.generate_random_solution()
    }
}
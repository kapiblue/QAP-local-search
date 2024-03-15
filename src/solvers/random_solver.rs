use rand::rngs::ThreadRng;

use crate::qap_problem::QapProblem;
use crate::solvers::solver::Solver;
use crate::solution::Solution;
use crate::utils::*;

pub struct RandomSolver<'a> {
    problem: &'a QapProblem,
    rng: ThreadRng,
}

impl<'a> RandomSolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem) -> RandomSolver<'a> {
        let rng = rand::thread_rng();
        RandomSolver { problem, rng }
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
       let eval = solution.evaluate(self.problem.matrix_a_ref(), 
       self.problem.matrix_b_ref());
       solution
    }
    fn get_type(&self) -> &str {
        return "random"
    }
}
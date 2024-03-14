use crate::solution::Solution;
use crate::solvers::solver::Solver;

use csv::Writer;

pub struct Experiment<'a> 
{
    solver: &'a mut dyn Solver,
    n_runs: usize,
    solutions: Vec<Solution>,
    iterations: Vec<i32>,
}

impl <'a>Experiment<'a>{
    pub fn new(solver: &'a mut dyn Solver, n_runs: usize) -> Self {
        let mut solutions: Vec<Solution> = Vec::with_capacity(n_runs);
        let mut iterations: Vec<i32> = Vec::with_capacity(n_runs);
        Experiment {
            solver,
            n_runs,
            solutions,
            iterations,
        }
    }

    pub fn run(&mut self) -> () {
        for i in 0..self.n_runs{
            let solution = self.solver.solve();
            let n_iterations = 1;
            &self.solutions.push(solution);
            &self.iterations.push(n_iterations);
        }
    }

    pub fn save_results(&self, filename: String){
        for i in 0..self.n_runs{
            println!("Run {}, solution {:?}, eval {}, iter {}",
             i, self.solutions[i].solution_array,
             self.solutions[i].get_eval(),
             self.iterations[i]);
        }
    }
}

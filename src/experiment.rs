use crate::solution::Solution;
use crate::solvers::solver::{Solver};

use csv::Writer;
use std::error::Error;
use std::time::Instant;

pub struct Experiment<'a> {
    solver: &'a mut dyn Solver,
    n_runs: usize,
    solutions: Vec<Solution>,
    iterations: Vec<i32>,
    average_time: u128,
}

impl<'a> Experiment<'a> {
    pub fn new(solver: &'a mut dyn Solver, n_runs: usize) -> Self {
        let solutions: Vec<Solution> = Vec::with_capacity(n_runs);
        let iterations: Vec<i32> = Vec::with_capacity(n_runs);
        let average_time: u128 = 0;
        Experiment {
            solver,
            n_runs,
            solutions,
            iterations,
            average_time,
        }
    }

    pub fn run(&mut self) -> () {
        let start = Instant::now();
        for _ in 0..self.n_runs {
            let solution = self.solver.solve();
            let n_iterations = 1;
            self.solutions.push(solution);
            self.iterations.push(n_iterations);
        }
        let duration = start.elapsed();
        self.average_time = duration.as_nanos();
    }

    pub fn print_results(&self) {
        for i in 0..self.n_runs {
            println!(
                "Run {}, solution {:?}, eval {}, iter {}",
                i,
                self.solutions[i].solution_array,
                self.solutions[i].get_eval(),
                self.iterations[i]
            );
        }
    }
    pub fn save_results(&self, path: &String) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(path)?;
        // Write column names
        wtr.write_record(&["run", "solution", "evaluation", "iteration", "time"])?;
        for i in 0..self.n_runs {
            wtr.write_record(&[
                i.to_string(),
                format!("{:?}", &self.solutions[i].solution_array),
                self.solutions[i].get_eval().to_string(),
                self.iterations[i].to_string(),
                self.average_time.to_string(),
            ])?;
        }
        wtr.flush()?;
        Ok(())
    }
}

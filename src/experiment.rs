use crate::solution::Solution;
use crate::solvers::solver::Solver;

use csv::Writer;
use std::error::Error;
use std::time::Instant;

pub struct Experiment<'a> {
    solver: &'a mut dyn Solver,
    n_runs: usize,
    solutions: Vec<Solution>,
    iterations: Vec<i32>,
    elapsed_time: Vec<u128>,
}

impl<'a> Experiment<'a> {
    pub fn new(solver: &'a mut dyn Solver, n_runs: usize) -> Self {
        let solutions: Vec<Solution> = Vec::with_capacity(n_runs);
        let iterations: Vec<i32> = Vec::with_capacity(n_runs);
        let elapsed_time: Vec<u128> = Vec::with_capacity(n_runs);
        Experiment {
            solver,
            n_runs,
            solutions,
            iterations,
            elapsed_time,
        }
    }
    // Run solver n times
    pub fn run(&mut self) -> () {
        for i in 0..self.n_runs {
            let start = Instant::now();

            let solution = self.solver.solve();
            let n_iterations = self.solver.get_iter_count();
            let elapsed = start.elapsed().as_millis();
            println!("Algorithm iteration {}; Best found solution evaluation {}", i, solution.get_eval());
            self.solutions.push(solution);
            self.iterations.push(n_iterations);
            self.elapsed_time.push(elapsed);
            
        }
    }

    // Run solver with time limit. Each of the n runs is limited.
    pub fn run_with_timelimit(&mut self, limit: u128) -> () {
        for _ in 0..self.n_runs {
            let start = Instant::now();
            let mut n_iterations = 0;
            let mut elapsed = 0;
            while elapsed < limit {
                elapsed = start.elapsed().as_millis();
                let solution = self.solver.solve();
                n_iterations = n_iterations + 1;
                println!("Algorithm iteration {}; Best found solution evaluation {}", n_iterations, solution.get_eval());
                self.solutions.push(solution);
            }
            self.iterations.push(n_iterations);
            self.elapsed_time.push(elapsed);
        }
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
        wtr.write_record(&["run", "solution", "evaluation", "iterations", "time"])?;
        for i in 0..self.n_runs {
            wtr.write_record(&[
                i.to_string(),
                format!("{:?}", &self.solutions[i].solution_array),
                self.solutions[i].get_eval().to_string(),
                self.iterations[i].to_string(),
                self.elapsed_time[i].to_string(),
            ])?;
        }
        wtr.flush()?;
        Ok(())
    }
}

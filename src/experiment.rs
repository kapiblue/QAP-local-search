use crate::solution::Solution;
use crate::solvers::solver::Solver;

use csv::Writer;
use std::error::Error;
use std::time::Instant;
use meansd::MeanSD;

/// An experiment object stores a solver, runs an experiment
/// measures time, and saves a csv file with results
pub struct Experiment<'a> {
    solver: &'a mut dyn Solver,
    /// How many times the experiment should be ran
    n_runs: usize,
    /// To store final solutions
    final_solutions: Vec<Solution>,
    /// To store initial solutions
    initial_solutions: Vec<Option<Solution>>,
    iterations: Vec<i32>,
    updates: Vec<i32>,
    elapsed_time: Vec<u128>,
}

impl<'a> Experiment<'a> {
    pub fn new(solver: &'a mut dyn Solver, n_runs: usize) -> Self {
        let final_solutions: Vec<Solution> = Vec::with_capacity(n_runs);
        let initial_solutions: Vec<Option<Solution>> = Vec::with_capacity(n_runs);
        let iterations: Vec<i32> = Vec::with_capacity(n_runs);
        let updates: Vec<i32> = Vec::with_capacity(n_runs);
        let elapsed_time: Vec<u128> = Vec::with_capacity(n_runs);
        Experiment {
            solver,
            n_runs,
            final_solutions,
            initial_solutions,
            iterations,
            updates,
            elapsed_time,
        }
    }
    /// Runs solver n times, measures time,
    /// and saves statistics
    pub fn run(&mut self) -> () {
        for i in 0..self.n_runs {

            let start = Instant::now();
            let solution = self.solver.solve();
            let initial_solution = self.solver.get_initial_solution();
            let elapsed = start.elapsed().as_millis();

            let n_iterations = self.solver.get_iter_count();
            let n_updates = self.solver.get_update_count();
            println!("Algorithm iteration {}; Best found solution evaluation {}", i, solution.get_eval());
            self.final_solutions.push(solution);
            self.initial_solutions.push(initial_solution);
            self.iterations.push(n_iterations);
            self.updates.push(n_updates);
            self.elapsed_time.push(elapsed);
            
        }
    }

    /// Runs solver with time limit. Each of the n runs is limited.
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
                self.final_solutions.push(solution);
            }
            self.iterations.push(n_iterations);
            self.elapsed_time.push(elapsed);
        }
    }

    /// Returns the mean elapsed time of all runs
    /// The run finction should be ran first.
    pub fn get_mean_elapsed_time(&self) -> u128 {
        return self.elapsed_time.iter().sum::<u128>() / self.elapsed_time.len() as u128
    }

    /// Returns the mean and standars deviation of final evaluations of all runs
    /// The run finction should be ran first.
    pub fn get_elapsed_time_mean_std(&self) -> (f64, f64) {
        let mut meansd = MeanSD::default();
        for i in 0..self.n_runs{
            meansd.update(self.elapsed_time[i] as f64)
        }
        return (meansd.mean(), meansd.sstdev())
    }

    /// Returns the mean and standars deviation of final evaluations of all runs
    /// The run finction should be ran first.
    pub fn get_final_evaluation_mean_std(&self) -> (f64, f64) {
        let mut meansd = MeanSD::default();
        for i in 0..self.n_runs{
            meansd.update(self.final_solutions[i].get_eval() as f64)
        }
        return (meansd.mean(), meansd.sstdev())
    }

    pub fn print_results(&self) {
        for i in 0..self.n_runs {
            println!(
                "Run {}, solution {:?}, eval {}, iter {}",
                i,
                self.final_solutions[i].solution_array,
                self.final_solutions[i].get_eval(),
                self.iterations[i]
            );
        }
    }
    /// Saves results in a csv file  according to the provided path
    pub fn save_results(&self, path: &String) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(path)?;
        // Write column names
        wtr.write_record(&["run", "final_solution", "final_evaluation", "initial_solution",
                           "initial_evaluation", "iterations", "updates","time"])?;
        for i in 0..self.n_runs {
            let mut initial_solution_array: &Vec<usize>= &vec![0; 1];
            let mut initial_solution_eval: i32 = 100000000;
            if !self.initial_solutions[i].is_none(){
                initial_solution_array = &self.initial_solutions[i].as_ref().unwrap().solution_array;
                initial_solution_eval = self.initial_solutions[i].as_ref().unwrap().get_eval();
            }
            wtr.write_record(&[
                i.to_string(),
                format!("{:?}", &self.final_solutions[i].solution_array),
                self.final_solutions[i].get_eval().to_string(),
                format!("{:?}", initial_solution_array),
                initial_solution_eval.to_string(),
                self.iterations[i].to_string(),
                self.updates[i].to_string(),
                self.elapsed_time[i].to_string(),
            ])?;
        }
        wtr.flush()?;
        Ok(())
    }
}

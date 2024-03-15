use qap_problem::*;
use utils::*;

mod experiment;
mod qap_problem;
mod solution;
mod solvers;
mod utils;

use solvers::local_search_solver::LocalSearchSolver;
use solvers::random_solver::RandomSolver;
use solvers::steepest_ls_solver::SteepestLSSolver;

use crate::experiment::Experiment;

use std::path::Path;

const INSTANCES: [&str; 1] = ["chr12a.dat"];
const RESULTS_FOLDER: &str = "results";
// This folder should be inside ./data
const DATA_FOLDER: &str = "qapdatsol";

fn main() {
    for instance_filename in INSTANCES {
        let instance_path = Path::new(".")
            .join("data")
            .join(DATA_FOLDER)
            .join(instance_filename)
            .to_string_lossy()
            .to_string();
        match QapProblem::new(&instance_path) {
            Ok(qap_problem) => {
                println!("Matrix A:");
                print_matrix(qap_problem.matrix_a_ref());
                println!("Matrix B:");
                print_matrix(qap_problem.matrix_b_ref());

                // init random solver
                let mut random_solver: RandomSolver<'_> = RandomSolver::new(&qap_problem);
                let mut experiment = Experiment::new(&mut random_solver, 10);
                // Run with a time limit of 10 milliseconds
                experiment.run_with_timelimit(10);
                let path = Path::new(".")
                    .join(RESULTS_FOLDER)
                    .join(instance_filename.to_owned() + "_random.csv")
                    .to_string_lossy()
                    .to_string();
                let _ = experiment.save_results(&path);

                let mut ls_solver: LocalSearchSolver<'_> = LocalSearchSolver::new(&qap_problem);
                let mut experiment = Experiment::new(&mut ls_solver, 10);
                // Run LS greedy for 10 times
                experiment.run();
                let path = Path::new(".")
                    .join(RESULTS_FOLDER)
                    .join(instance_filename.to_owned() + "_ls_greedy.csv")
                    .to_string_lossy()
                    .to_string();
                let _ = experiment.save_results(&path);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

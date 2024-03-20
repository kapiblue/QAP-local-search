use qap_problem::*;
use utils::*;

mod experiment;
mod qap_problem;
mod solution;
mod solvers;
mod utils;

use solvers::greedy_ls_solver::GreedyLSSolver;
use solvers::random_solver::RandomSolver;
use solvers::steepest_ls_solver::SteepestLSSolver;

use crate::experiment::Experiment;
use crate::solvers::greedy_heuristic_solver::HeuristicSolver;
use crate::solvers::random_walk_solver::RandomWalkSolver;

use std::path::Path;

const INSTANCES: [&str; 8] = [
                              "bur26c.dat",
                              "chr12a.dat",
                              "els19.dat",
                              "tai80.dat",
                              "esc128.dat",
                              "had20.dat",
                              "nug30.dat",
                              "nug15.dat"];
                            
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
        println!("{}", instance_filename);
        match QapProblem::new(&instance_path) {
            Ok(qap_problem) => {
                println!("Matrix A:");
                print_matrix(qap_problem.matrix_a_ref());
                println!("Matrix B:");
                print_matrix(qap_problem.matrix_b_ref());

                // init random solver
                // let mut random_solver: RandomSolver<'_> = RandomSolver::new(&qap_problem);
                // let mut experiment = Experiment::new(&mut random_solver, 10);
                // // Run with a time limit of 10 milliseconds
                // experiment.run_with_timelimit(10);
                // let path = Path::new(".")
                //     .join(RESULTS_FOLDER)
                //     .join(instance_filename.to_owned() + "_random.csv")
                //     .to_string_lossy()
                //     .to_string();
                // let _ = experiment.save_results(&path);

                // let mut ls_solver: GreedyLSSolver<'_> = GreedyLSSolver::new(&qap_problem);
                // let mut experiment = Experiment::new(&mut ls_solver, 10);
                // // Run LS greedy for 10 times
                // experiment.run();
                // let path = Path::new(".")
                //     .join(RESULTS_FOLDER)
                //     .join(instance_filename.to_owned() + "_ls_greedy.csv")
                //     .to_string_lossy()
                //     .to_string();
                // let _ = experiment.save_results(&path);
                // let mut ls_solver: SteepestLSSolver<'_> = SteepestLSSolver::new(&qap_problem);
                // let mut experiment = Experiment::new(&mut ls_solver, 1);
                // // Run LS greedy for 10 times
                // experiment.run_with_timelimit(300);
                // // experiment.run();
                // let path = Path::new(".")
                //     .join(RESULTS_FOLDER)
                //     .join(instance_filename.to_owned() + "_ls_greedy.csv")
                //     .to_string_lossy()
                //     .to_string();
                // let _ = experiment.save_results(&path);

                println!("Random Solver");
                let mut ls_solver: RandomSolver<'_> = RandomSolver::new(&qap_problem, Some(1000));
                let mut experiment = Experiment::new(&mut ls_solver, 10);
                experiment.run();
                let path = Path::new(".")
                    .join(RESULTS_FOLDER)
                    .join(instance_filename.to_owned() + "_random.csv")
                    .to_string_lossy()
                    .to_string();
                let _ = experiment.save_results(&path);

                println!("Random Walk Solver");
                let mut ls_solver: RandomWalkSolver<'_> =
                    RandomWalkSolver::new(&qap_problem, Some(1000));
                let mut experiment = Experiment::new(&mut ls_solver, 10);
                // Run LS greedy for 10 times
                experiment.run();
                // experiment.run();
                let path = Path::new(".")
                    .join(RESULTS_FOLDER)
                    .join(instance_filename.to_owned() + "_random_walk.csv")
                    .to_string_lossy()
                    .to_string();
                let _ = experiment.save_results(&path);

                println!("Heuristic Solver");
                let mut ls_solver: HeuristicSolver<'_> = HeuristicSolver::new(&qap_problem);
                let mut experiment = Experiment::new(&mut ls_solver, 10);
                // Run Heuristic for 10 times
                experiment.run();
                // experiment.run();
                let path = Path::new(".")
                    .join(RESULTS_FOLDER)
                    .join(instance_filename.to_owned() + "_heuristic.csv")
                    .to_string_lossy()
                    .to_string();
                let _ = experiment.save_results(&path);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

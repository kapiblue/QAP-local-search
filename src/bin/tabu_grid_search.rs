use qap_local_search::experiment::Experiment;
use qap_local_search::qap_problem::QapProblem;

use qap_local_search::solvers::tabu_search_solver::TSSolver;

use core::cmp::max;
use core::cmp::min;
use csv::Writer;
use std::path::Path;

/// Instance file names
const INSTANCES: [&str; 8] = [
    "tai60a.dat",
    "wil100.dat",
    "bur26c.dat",
    "els19.dat",
    "esc128.dat",
    "had20.dat",
    "nug15.dat",
    "nug30.dat",
];

/// The name of the folder where ths csv files should be saved
const RESULTS_FOLDER: &str = "tabu_gs_results";
/// Folder with instances. This folder should be inside ./data
const DATA_FOLDER: &str = "qapdatsol";
/// How many times the experiments should be ran (for statistics)
const NRUNS: usize = 10;

// Grid Search parameters
const TENURES: [f32; 3] = [0.2, 0.25, 0.3];
const KS: [f32; 3] = [0.1, 0.25, 0.4];
const LITS: [i32; 3] = [10, 25, 50];

/// Runs experiments across various solvers (algorithms)
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
            Ok(mut qap_problem) => {
                // Set path to save the result
                let path = Path::new(".")
                    .join(RESULTS_FOLDER)
                    .join(instance_filename.to_owned() + "_tabu_gs.csv")
                    .to_string_lossy()
                    .to_string();
                // Initialize a CSV Writer
                let mut wtr = Writer::from_path(&path).unwrap();
                // Write column names
                let _ = wtr.write_record(&[
                    "tenure",
                    "k",
                    "lack_impr_iter",
                    "eval_mean",
                    "eval_std",
                    "time_mean",
                    "time_std",
                ]);
                let N = qap_problem.get_n();

                for t in TENURES {
                    for k in KS {
                        for lit in LITS {
                            // Calculate the parameters as integers
                            let int_t = (N as f32 * t) as i32;
                            let int_k = max((N as f32 * k ) as usize, 1);
                            /// let lit = min(int_t - 2, lit);
                            println!("Tabu Search Solver t = {}, k = {}, lit = {}", t, k, lit);
                            println!("int_t = {}, int_k = {}", int_t, int_k);
                            let mut ts_solver: TSSolver<'_> =
                                TSSolver::new(&mut qap_problem, int_t, 1.0, int_k, lit);
                            let mut experiment = Experiment::new(&mut ts_solver, NRUNS);
                            experiment.run();
                            // Collect the experiment statistics
                            let (eval_mean, eval_std) = experiment.get_final_evaluation_mean_std();
                            let (time_mean, time_std) = experiment.get_elapsed_time_mean_std();
                            // Write parameters and results to csv
                            let _ = wtr.write_record(&[
                                int_t.to_string(), int_k.to_string(), lit.to_string(),
                                eval_mean.to_string(), eval_std.to_string(),
                                time_mean.to_string(), time_std.to_string(),
                            ]);
                        }
                    }
                }
                let _ = wtr.flush();
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

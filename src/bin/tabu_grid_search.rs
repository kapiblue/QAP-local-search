use qap_local_search::experiment::Experiment;
use qap_local_search::qap_problem::QapProblem;

use qap_local_search::solvers::tabu_search_solver::TSSolver;

use std::path::Path;
use core::cmp::max;

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

const TENURES: [f32; 3] = [0.2, 0.25, 0.3];
const KS: [f32; 3] = [0.3, 0.5, 0.7];

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
                let N = qap_problem.get_n();

                for t in TENURES {
                    for k in KS {
                        let int_t = (N as f32 * t) as i32;
                        let int_k = max((N as f32 * k) as usize, 5);
                        let lit = int_t - 2;
                        println!("Tabu Search Solver t = {}, k = {}, lit = {}", t, k, lit);
                        println!("int_t = {}, int_k = {}", int_t, int_k);
                        let mut ts_solver: TSSolver<'_> =
                            TSSolver::new(&mut qap_problem, int_t, 1.0, int_k, lit);
                        let mut experiment = Experiment::new(&mut ts_solver, NRUNS);
                        experiment.run();
                        let path = Path::new(".")
                            .join(RESULTS_FOLDER)
                            .join(
                                instance_filename.to_owned()
                                    + "_tabu"
                                    + "_t"
                                    + &t.to_string()
                                    + "_k"
                                    + &k.to_string()
                                    + "_l"
                                    + &lit.to_string()
                                    + "_.csv",
                            )
                            .to_string_lossy()
                            .to_string();
                        let _ = experiment.save_results(&path);
                    }
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

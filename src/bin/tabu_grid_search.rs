use qap_local_search::qap_problem::QapProblem;
use qap_local_search::experiment::Experiment;

use qap_local_search::solvers::tabu_search_solver::TSSolver;

use std::path::Path;

/// Instance file names
const INSTANCES: [&str; 8] = [
    "nug30.dat",
    "tai60a.dat",
    "wil100.dat",
    "bur26c.dat",
    "els19.dat",
    "esc128.dat",
    "had20.dat",
    "nug15.dat",
];

/// The name of the folder where ths csv files should be saved
const RESULTS_FOLDER: &str = "results";
/// Folder with instances. This folder should be inside ./data
const DATA_FOLDER: &str = "qapdatsol";
/// How many times the experiments should be ran (for statistics)
const NRUNS: usize = 10;




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
                // TABU

                println!("Tabu Search Solver");
                let mut ts_solver: TSSolver<'_> = TSSolver::new(&mut qap_problem,  20, 0.9, 20, 10);
                let mut experiment = Experiment::new(&mut ts_solver, NRUNS);
                experiment.run();
                let path = Path::new(".")
                    .join(RESULTS_FOLDER)
                    .join(instance_filename.to_owned() + "_tabu.csv")
                    .to_string_lossy()
                    .to_string();
                let _ = experiment.save_results(&path);
                break;

                // // GREEDY
                // println!("Greedy Solver");
                // let mut ls_solver: GreedyLSSolver<'_> = GreedyLSSolver::new(&qap_problem);
                // let mut experiment = Experiment::new(&mut ls_solver, NRUNS);
                // experiment.run();
                // let path = Path::new(".")
                //     .join(RESULTS_FOLDER)
                //     .join(instance_filename.to_owned() + "_greedy.csv")
                //     .to_string_lossy()
                //     .to_string();
                // let _ = experiment.save_results(&path);
                // // Collect greedy running time
                // let greedy_time = experiment.get_mean_elapsed_time();

                // // STEEPEST
                // println!("Steepest Solver");
                // let mut ls_solver: SteepestLSSolver<'_> = SteepestLSSolver::new(&qap_problem);
                // let mut experiment = Experiment::new(&mut ls_solver, NRUNS);
                // experiment.run();
                // let path = Path::new(".")
                //     .join(RESULTS_FOLDER)
                //     .join(instance_filename.to_owned() + "_steepest.csv")
                //     .to_string_lossy()
                //     .to_string();
                // let _ = experiment.save_results(&path);
                // let steepest_time = experiment.get_mean_elapsed_time();

                // let time_limit = (greedy_time + steepest_time) / 2 as u128;
                // // println!("{}", time_limit);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

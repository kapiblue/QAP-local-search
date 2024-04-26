use qap_local_search::experiment::Experiment;
use qap_local_search::qap_problem::QapProblem;

use qap_local_search::solvers::simulated_annealing_solver::SASolver;
use qap_local_search::utils::parse_json;

use std::path::Path;

/// Instance file names
const INSTANCES: [&str; 8] = [
    "bur26c.dat",
    "esc128.dat",
    "nug30.dat",
    "tai60a.dat",
    "wil100.dat",
    "els19.dat",
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

        // Read json config
        let config_path = Path::new(".")
        .join("configs")
        .join("sa.json").to_string_lossy()
        .to_string();

        let config = parse_json(&config_path);
        match QapProblem::new(&instance_path) {
            Ok(mut qap_problem) => {
                println!("Simulated Annealing Solver");
                let params = config.get(instance_filename).unwrap();
                let temp_mul = params.get("temp_mul").unwrap().as_f64().unwrap() as f32;
                let iter_mul = params.get("iter_mul").unwrap().as_u64().unwrap() as usize;
                let l_div = params.get("l_div").unwrap().as_i64().unwrap() as i32;
                println!("Params: temp_mul = {}, iter_mul = {}, l_div = {}", temp_mul, iter_mul, l_div);
                println!("Simulated Annealing Solver");
                let mut sa_solver: SASolver<'_> = SASolver::new(&mut qap_problem, temp_mul, iter_mul, l_div);
                sa_solver.compute_initial_temperature();
                let mut experiment = Experiment::new(&mut sa_solver, NRUNS);
                experiment.run();
                let path = Path::new(".")
                    .join(RESULTS_FOLDER)
                    .join(instance_filename.to_owned() + "_sa.csv")
                    .to_string_lossy()
                    .to_string();
                let _ = experiment.save_results(&path);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

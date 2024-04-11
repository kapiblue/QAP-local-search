use qap_local_search::experiment::Experiment;
use qap_local_search::qap_problem::QapProblem;

use qap_local_search::solvers::solver::Solver;
use qap_local_search::solvers::tabu_search_solver::TSSolver;
use qap_local_search::utils::parse_json;

use std::path::Path;

/// Instance file names
const INSTANCES: [&str; 8] = [
    "esc128.dat",
    "bur26c.dat",
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
        .join("tabu.json").to_string_lossy()
        .to_string();
;
        let config = parse_json(&config_path);
        match QapProblem::new(&instance_path) {
            Ok(mut qap_problem) => {
                println!("Tabu Search Solver");
                let params = config.get(instance_filename).unwrap();
                let tenure = params.get("tenure").unwrap().as_i64().unwrap() as i32;
                let k = params.get("k").unwrap().as_u64().unwrap() as usize;
                let lack_impr_iter = params.get("lack_impr_iter").unwrap().as_i64().unwrap() as i32;
                println!("Params: tenure = {}, k= {}, lack iter = {}", tenure, k, lack_impr_iter);
                let mut ts_solver: TSSolver<'_> = TSSolver::new(&mut qap_problem, tenure, 0.8, k, lack_impr_iter);
                // let sol = ts_solver.solve();
                // println!("Eval {}", sol.get_eval());
                let mut experiment = Experiment::new(&mut ts_solver, NRUNS);
                experiment.run();
                let path = Path::new(".")
                    .join(RESULTS_FOLDER)
                    .join(instance_filename.to_owned() + "_tabu.csv")
                    .to_string_lossy()
                    .to_string();
                let _ = experiment.save_results(&path);
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

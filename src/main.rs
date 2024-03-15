use std::path::Path;


use qap_problem::*;
use solution::*;
use utils::*;

mod qap_problem;
mod solution;
mod experiment;
mod utils;
mod solvers;

use solvers::random_solver::RandomSolver;
use solvers::local_search_solver::LocalSearchSolver;
use solvers::steepest_ls_solver::SteepestLSSolver;
use solvers::solver::Solver;

use crate::experiment::Experiment;


fn main() {
    // Create a random generator to reuse
    let mut rng = rand::thread_rng();
    // Create an array of zeros
    let mut array: [usize; 100] = [0; 100];
    println!("Zero list:");
    print_array(&array);
    // Fill the array with ordered values
    arange(&mut array, 0, 1);
    println!("Ordered list:");
    print_array(&array);
    // Randomize the array
    permute_array(&mut rng, &mut array);
    println!("Random permutation:");
    print_array(&array);
    let (x1, x2) = generate_random_int_pair(&mut rng, 8);
    println!("Random pair: {}, {}", x1, x2);

    // Test reading a sample file
    match QapProblem::new("data/qapdatsol/chr12a.dat") {
        Ok(sample_qap) => {
            println!("Matrix A:");
            print_matrix(sample_qap.matrix_a_ref());
            println!("Matrix B:");
            print_matrix(sample_qap.matrix_b_ref());
            
            // init random solver
            let mut random_solver: RandomSolver<'_> = RandomSolver::new(&sample_qap);

            // call solve() method using random solver
            let mut solution: Solution = random_solver.solve();
            let cost = solution.evaluate(
                sample_qap.matrix_a_ref(),
                sample_qap.matrix_b_ref()
            );

            println!("Random solution: {}", solution);


            let delta = solution.calculate_delta(sample_qap.matrix_a_ref(),
            sample_qap.matrix_b_ref(), &[1,5]);

            println!("Delta: {}", delta);

            solution.exchange_facilities(&[1,5]);
            let cost_ex = solution.evaluate(
                sample_qap.matrix_a_ref(),
                sample_qap.matrix_b_ref()
            );
            
            println!("Exchanged solution: {}", solution);

            println!("Error: {}", cost_ex - (cost + delta));

            let mut ls_solver: LocalSearchSolver<'_> = LocalSearchSolver::new(&sample_qap);
            let ls_solution = ls_solver.solve(solution);
            println!("Exchanged solution: {}", ls_solution);

            // init random solver
            let mut random_solver: RandomSolver<'_> = RandomSolver::new(&sample_qap);

            // call solve() method using random solver
            let mut solution: Solution = random_solver.solve();
            let mut steepest_solver: SteepestLSSolver<'_> = SteepestLSSolver::new(&sample_qap);
            let steepest_solution = steepest_solver.solve(solution);
            println!("Steepest solution: {}", steepest_solution);

            let mut experiment = Experiment::new(&mut random_solver, 100);
            let path = Path::new("..").join("results").join("random.csv");
            experiment.run();
            experiment.save_results(path);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

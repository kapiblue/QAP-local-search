use qap_problem::*;
use solution::*;
use utils::*;

mod qap_problem;
mod solution;
mod utils;

mod solvers;
use solvers::random_solver::RandomSolver;
use solvers::solver::Solver;

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
            let random_solver: RandomSolver<'_> = RandomSolver::new(&sample_qap);

            // call solve() method using random solver
            let solution: Solution = random_solver.solve();

            println!("Solution array: {:?}", solution.solution_array);

            let cost = solution.evaluate(
                sample_qap.matrix_a_ref(),
                sample_qap.matrix_b_ref()
            );

            println!("Evaluation of the optimum: {}", cost);

            // // Genrate a vector o pairs
            // let mut pairs = generate_pairs(sample_qap.get_n());
            // // Randomly permute the vector
            // permute_array(&mut rng, &mut pairs);
            // // Print for verification
            // for row in pairs {
            //     println!("{:?}", row);
            // }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

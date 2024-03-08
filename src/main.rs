use qap_problem::*;
use solution::*;
use utils::*;

mod qap_problem;
mod solution;
mod utils;

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
            let optimal_solution: [usize; 12] = [7, 5, 12, 2, 1, 3, 9, 11, 10, 6, 8, 4];
            let eval: i32 = evaluate(
                &optimal_solution,
                sample_qap.matrix_a_ref(),
                sample_qap.matrix_b_ref(),
            );
            println!("Optimal solution:");
            print_array(&optimal_solution);
            println!("Evaluation of the optimum: {}", eval);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

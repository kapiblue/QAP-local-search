use utils::*;

mod utils;

fn main() {
    // Create a random generator to reuse
    let mut rng = rand::thread_rng();
    // Create an array of zeros
    let mut array: [u32; 100] = [0; 100];
    println!("Zero list:");
    print_array(&array);
    // Fill the array with ordered values
    arange(&mut array, 0, 1);
    println!("\nOrdered list:");
    print_array(&array);
    // Randomize the array
    permute_array(&mut rng, &mut array);
    println!("\nRandom permutation:");
    print_array(&array);
    let (x1, x2) = generate_random_int_pair(&mut rng, 8);
    println!("\nRandom pair: {}, {}", x1, x2);

    // Test reading a sample file
    let filename = "data/qapdatsol/chr12a.dat";

    match parse_file(filename) {
        Ok(matrices) => {
            println!("Matrix A:");
            for row in &matrices.a {
                println!("{:?}", row);
            }

            println!("Matrix B:");
            for row in &matrices.b {
                println!("{:?}", row);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

use utils::permute_array;
use utils::print_array;

mod utils;

fn main() {
    let mut array: [usize; 100] = core::array::from_fn(|i| i + 1);
    println!("Ordered list:");
    print_array(&array);
    permute_array(&mut array);
    println!("\nRandom permutation:");
    print_array(&array);
}

use num_traits::Num;
use rand::Rng;
use std::{fs::File, path};

/// The `permute_array` function in Rust shuffles the elements of an array using a random number
/// generator.
///
/// Arguments:
///
/// * `rng`: The `rng` parameter is a mutable reference to a random number generator (`Rng`) that is
/// used to generate random numbers for shuffling the elements of the input array.
/// * `array`: The `array` parameter in the `permute_array` function is a mutable slice of elements of
/// type `T`.
pub fn permute_array<R: Rng + ?Sized, T>(rng: &mut R, array: &mut [T])
where
    T: Copy,
{
    // Generate a random integer
    let len = (&array).len();
    let mut initial_range: usize = len;
    let mut tmp: T;
    let mut exchange_index: usize;
    for _i in 0..array.len() {
        exchange_index = rng.gen_range(0..initial_range);
        initial_range -= 1;
        tmp = array[initial_range];
        array[initial_range] = array[exchange_index];
        array[exchange_index] = tmp;
    }
}

/// The function generates a pair of random integers within a specified range.
///
/// Arguments:
///
/// * `rng`: The `rng` parameter is a mutable reference to a type that implements the `Rng` trait. This
/// trait is typically used for generating random numbers.
/// * `range`: The `range` parameter specifies the upper limit (exclusive) for generating random
/// integers. The function `generate_random_int_pair` will generate two random integers within the range
/// of 0 to `range` (exclusive).
pub fn generate_random_int_pair<R: Rng + ?Sized>(rng: &mut R, range: u32) -> (u32, u32) {
    let x1: u32 = rng.gen_range(0..range);
    let x2: u32 = (rng.gen_range(0..range - 1) + 1 + x1) % range;

    return (x1, x2);
}

/// Generates all pairs (i,j) i!=j where i=0..n, j=i..n.
/// Returns a vector of pairs
pub fn generate_pairs(n: usize) -> Vec<[usize; 2]> {
    let mut pairs = Vec::with_capacity(n);
    for i in 0..n {
        for j in i + 1..n {
            pairs.push([i, j]);
        }
    }
    pairs
}

/// Generates all two pairs [(i, j), (k, l)] where i!=j, k!=l, i!=k, j!=l
/// Returns a vector of pairs
pub fn generate_two_pairs(n: usize) -> Vec<Vec<[usize; 2]>> {
    let mut pairs = Vec::with_capacity(n);
    for i in 0..n {
        for j in i + 1..n {
            for k in 0..n {
                for l in k + 1..n {
                    if i != k && i != l && j != k && j != l {
                        pairs.push([[i, j], [k, l]].to_vec());
                    }
                }
            }
        }
    }
    pairs
}

/// The function `arange` populates a mutable array with values starting from a given low value and
/// incrementing by a specified step.
///
/// Arguments:
///
/// * `array`: The `array` parameter is a mutable slice of elements of type `T`. The function `arange`
/// will populate this array with values starting from the `low` value and incrementing by the `step`
/// value for each element in the array.
/// * `low`: The `low` parameter represents the starting value for the range.
/// * `step`: The `step` parameter represents the increment value used to generate the elements in the
/// array. It determines the difference between consecutive elements in the array.
pub fn arange<T: Num>(array: &mut [T], low: T, step: T)
where
    T: Copy,
{
    let mut current: T = low;
    for i in 0..array.len() {
        array[i] = current;
        current = current + step;
    }
}

/// Prints each element of a generic array separated by a space.
///
/// Arguments:
///
/// * `array`: The `print_array` function takes a slice of type `T`, where `T` implements the `Display`
/// trait from the standard library. The function iterates over the elements of the slice and prints
/// each element followed by a space.
pub fn print_array<T>(array: &[T])
where
    T: std::fmt::Display + std::fmt::Debug,
{
    for value in array {
        print!("{:?} ", value);
    }
    println!();
}
/// Prints a two-dimensional vector
pub fn print_matrix<T>(matrix: &Vec<Vec<T>>)
where
    T: std::fmt::Debug,
{
    for row in matrix {
        println!("{:?}", row);
    }
}
/// Reads a json file from path
pub fn parse_json(config_path: &str) -> serde_json::Value {
    let file = File::open(config_path).expect("The json file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("The json file should be proper JSON");
    json
}

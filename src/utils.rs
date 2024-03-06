use num_traits::Num;
use rand::Rng;
use std::time::Instant;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    T: std::fmt::Display,
{
    for i in 0..array.len() {
        print!("{} ", array[i]);
    }
}

// pub fn timeit<F: Fn() -> T, T>(f: F, i: u32) -> T {
//     let start = Instant::now();
//     let result = f();
//     let duration = start.elapsed();
//     println!("it took {} seconds", duration.as_millis());
//     result
//   }


// Define a struct for matrix A and B
pub struct Matrices {
    pub a: Vec<Vec<i32>>,
    pub b: Vec<Vec<i32>>,
}


// Function to parse the file
pub fn parse_file(filename: &str) -> Result<Matrices, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Parse the first line to get the size of the matrices
    let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

    // Skip the empty line
    lines.next();

    // Parse matrix A
    let mut matrix_a = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap()?;
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        matrix_a.push(row);
    }

    // Skip the empty line
    lines.next();

    // Parse matrix B
    let mut matrix_b: Vec<Vec<i32>> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut row: Vec<i32> = Vec::with_capacity(n);
        let mut count: usize = 0;
        let mut line: String = String::new();
        loop {
            line += &lines.next().unwrap()?;
            let nums: Vec<&str> = line.split_whitespace().collect();
            for num in nums {
                if count < n {
                    row.push(num.parse().unwrap());
                    count += 1;
                } else {
                    break;
                }
            }
            if count >= n {
                break;
            }
        }
        matrix_b.push(row);
    }

    Ok(Matrices {
        a: matrix_a,
        b: matrix_b,
    })
}

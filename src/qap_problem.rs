use crate::solution::Solution;
use crate::utils::*;
use rand::rngs::ThreadRng;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct QapProblem {
    n: usize,
    matrix_a: Vec<Vec<i32>>,
    matrix_b: Vec<Vec<i32>>,
    rng: ThreadRng,
}

impl QapProblem {
    /// Constructor
    pub fn new(filename: &str) -> Result<QapProblem, std::io::Error> {
        // Create and return an object if no error
        match Self::parse_file(filename) {
            Ok((n, matrix_a, matrix_b)) => Ok(QapProblem {
                n: n,
                matrix_a: matrix_a,
                matrix_b: matrix_b,
                rng: rand::thread_rng(),
            }),
            Err(err) => Err(err),
        }
    }

    // TODO: this function can't be used for now as it requires
    // &mut reference. QAPProblem object is not mutable when passed
    // to the solvers.
    pub fn generate_random_solution(&mut self) -> Solution {
        let n = self.get_n();
        let mut solution_array = vec![0; n];
        arange(&mut solution_array, 0, 1);
        permute_array(&mut self.rng, &mut solution_array);
        Solution::new(solution_array)
    }

    /// Returns a reference to matrix a
    pub fn matrix_a_ref(&self) -> &Vec<Vec<i32>> {
        &self.matrix_a
    }
    /// Returns a reference to matrix b
    pub fn matrix_b_ref(&self) -> &Vec<Vec<i32>> {
        &self.matrix_b
    }
    /// Returns the instance size
    pub fn get_n(&self) -> usize {
        self.n
    }

    /// Function to parse the file describing the instance
    fn parse_file(filename: &str) -> Result<(usize, Vec<Vec<i32>>, Vec<Vec<i32>>), std::io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Parse the first line to get the size of the matrices
        let n: usize = lines.next().unwrap()?.trim().parse().unwrap();

        // Parse matrix A
        let mut matrix_a = Vec::with_capacity(n);
        let mut i = 0;
        while i < n {
            let line = lines.next().unwrap()?;
            // Handle empty lines
            if line.len() != 0 {
                let row: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                matrix_a.push(row);
                i += 1;
            }
        }

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
            assert_eq!(row.len(), n);
            matrix_b.push(row);
        }
        assert_eq!(matrix_a.len(), n);
        assert_eq!(matrix_b.len(), n);
        Ok((n, matrix_a, matrix_b))
    }
}


use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct QapProblem {
    filename: String,
    n: usize,
    matrix_a: Vec<Vec<i32>>,
    matrix_b: Vec<Vec<i32>>,
}

impl QapProblem {
    // Constructor
    pub fn new(filename: &str) -> Result<QapProblem, std::io::Error> {
        match Self::parse_file(filename) {
            Ok((n, matrix_a, matrix_b)) => Ok(QapProblem {
                filename: String::from(filename),
                n: n,
                matrix_a: matrix_a,
                matrix_b: matrix_b,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn matrix_a_ref(&self) -> &Vec<Vec<i32>> {
        &self.matrix_a
    }

    pub fn matrix_b_ref(&self) -> &Vec<Vec<i32>> {
        &self.matrix_b
    }

    // Function to parse the file
    fn parse_file(filename: &str) -> Result<(usize, Vec<Vec<i32>>, Vec<Vec<i32>>), std::io::Error> {
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

        Ok((n, matrix_a, matrix_b))
    }
}

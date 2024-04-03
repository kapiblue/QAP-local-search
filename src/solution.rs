// Evaluates the solution given the reference to the solution array
// and references to matrices a and b
// pub fn evaluate<V: num_traits::Num>(
//     solution: &[usize],
//     matrix_a: &Vec<Vec<V>>,
//     matrix_b: &Vec<Vec<V>>,
// ) -> V
// where
//     V: Copy,
// {
//     let n = solution.len();
//     let mut evaluation: V = num_traits::zero();
//     for i in 0..n {
//         for j in 0..n {
//             let facility1 = solution[i] - 1;
//             let facility2 = solution[j] - 1;
//             let location1 = i;
//             let location2 = j;

//             evaluation =
//                 evaluation + matrix_a[location1][location2] * matrix_b[facility1][facility2];
//         }
//     }
//     evaluation
// }

// TODO: After reviewing the below implementation, possibly the above
// method might be removed.

// Implemented class for the Solution with evaluate() method

use core::fmt;
use std::f32::INFINITY;

#[derive(Debug, Default, Clone)]
/// A solution is represented here
pub struct Solution {
    /// The array stores the permutation
    pub solution_array: Vec<usize>,
    /// Stores the evaluation of the solution
    eval: i32,
    /// Stores the instance size
    n: usize,
}

impl Solution {
    /// Constructor of the solution from an existing vector (permutation)
    pub fn new(solution_array: Vec<usize>) -> Self {
        let eval: i32 = std::f32::INFINITY as i32;
        let n = solution_array.len();
        Solution {
            solution_array,
            eval,
            n,
        }
    }
    /// Returns the evaluation.
    /// evaluate() must be called first.
    pub fn get_eval(&self) -> i32 {
        self.eval
    }
    /// Sets evaluation, saves computations
    /// if the evaluation is already known
    pub fn set_eval(&mut self, eval:i32) {
        self.eval = eval
    }
    /// Returns the solution array (vector)
    pub fn get_solution_array(&self) -> Vec<usize> {
        self.solution_array.clone()
    }

    /// Evaluates the solution based on the provided matrices A and B (references)
    pub fn evaluate(&mut self, matrix_a: &Vec<Vec<i32>>, matrix_b: &Vec<Vec<i32>>) -> i32 {
        let n: usize = self.solution_array.len();
        let mut evaluation: i32 = 0;

        for i in 0..n {
            let facility1 = self.solution_array[i];
            let location1 = i;
            for j in 0..n {
                let facility2 = self.solution_array[j];
                let location2 = j;

                evaluation =
                    evaluation + matrix_a[location1][location2] * matrix_b[facility1][facility2]
            }
        }
        self.eval = evaluation;
        evaluation
    }
    /// Swaps array elements at provided indices
    pub fn exchange_facilities(&mut self, pair: &[usize; 2]) {
        let p1 = pair[0];
        let p2 = pair[1];
        let tmp = self.solution_array[p1];
        self.solution_array[p1] = self.solution_array[p2];
        self.solution_array[p2] = tmp;
    }


    /// Calculates delta of a pair exchange in O(N) time
    pub fn calculate_delta(
        &self,
        matrix_a: &Vec<Vec<i32>>,
        matrix_b: &Vec<Vec<i32>>,
        pair: &[usize; 2],
    ) -> i32 {
        let mut delta: i32 = 0;
        let i = pair[0];
        let j = pair[1];

        let fi = self.solution_array[i];
        let fj = self.solution_array[j];

        delta = delta + (matrix_a[i][i] - matrix_a[j][j]) * (matrix_b[fj][fj] - matrix_b[fi][fi]);
        delta = delta + (matrix_a[i][j] - matrix_a[j][i]) * (matrix_b[fj][fi] - matrix_b[fi][fj]);
        for g in 0..self.n {
            if g == i || g == j {
                continue;
            }
            let fg = self.solution_array[g];

            delta =
                delta + (matrix_a[g][i] - matrix_a[g][j]) * (matrix_b[fg][fj] - matrix_b[fg][fi]);
            delta =
                delta + (matrix_a[i][g] - matrix_a[j][g]) * (matrix_b[fj][fg] - matrix_b[fi][fg]);
        }
        delta
    }

     /// Exchanges n facilities
    pub fn exchange_n_facilities(&mut self, pairs: &Vec<[usize; 2]>) -> () {
        for pair in pairs {
            self.exchange_facilities(pair);
        }
    }

    pub fn calculate_n_deltas(
        &self,
        matrix_a: &Vec<Vec<i32>>,
        matrix_b: &Vec<Vec<i32>>,
        pairs: &Vec<[usize; 2]>,
    ) -> i32 {
        let mut delta: i32 = 0;
        for pair in pairs {
            delta += self.calculate_delta(matrix_a, matrix_b, pair);
        }
        delta
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Solution: {:?}, evaluation {}",
            self.solution_array, self.eval
        )
    }
}

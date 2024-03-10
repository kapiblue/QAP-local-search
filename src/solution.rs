
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
pub struct Solution {
    pub solution_array: Vec<usize>,
}

impl Solution {
    // constructor
    pub fn new(solution_array: Vec<usize>) -> Self {
        Solution {solution_array}
    }

    // evaluate the solution based on the provided matrices A and B
    pub fn evaluate<V>(&self, matrix_a: &Vec<Vec<V>>, matrix_b: &Vec<Vec<V>>) -> V
    where
        V: num_traits::Num + Copy,
    {
        let n: usize = self.solution_array.len();
        let mut evaluation: V = num_traits::zero();

        for i in 0..n {
            for j in 0..n {
                let facility1 = self.solution_array[i] - 1;
                let facility2 = self.solution_array[j] - 1;
                let location1 = i;
                let location2 = j;

                evaluation = 
                    evaluation + matrix_a[location1][location2] * matrix_b[facility1][facility2]
            }
        }
        
        evaluation
    }
}

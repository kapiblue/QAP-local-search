
// Evaluates the solution given the reference to the solution array
// and references to matrices a and b
pub fn evaluate<V: num_traits::Num>(
    solution: &[usize],
    matrix_a: &Vec<Vec<V>>,
    matrix_b: &Vec<Vec<V>>,
) -> V
where
    V: Copy,
{
    let n = solution.len();
    let mut evaluation: V = num_traits::zero();
    for i in 0..n {
        for j in 0..n {
            let facility1 = solution[i] - 1;
            let facility2 = solution[j] - 1;
            let location1 = i;
            let location2 = j;

            evaluation =
                evaluation + matrix_a[location1][location2] * matrix_b[facility1][facility2];
        }
    }
    evaluation
}

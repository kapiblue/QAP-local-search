use crate::solution::Solution;

// Trait for all solvers, for now there is only the method
// solve(), but we may think what should be added later on
pub trait Solver {
    fn solve(&self) -> Solution;
}
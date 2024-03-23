use crate::solution::Solution;

// Trait for all solvers, for now there is only the method
// solve(), but we may think what should be added later on
pub trait Solver {
    fn solve(&mut self) -> Solution;
    fn get_iter_count(&self) -> i32;
    fn get_update_count(&self) -> i32;
    fn get_initial_solution(&self) -> Option<Solution>;
}
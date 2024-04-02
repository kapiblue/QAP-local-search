use rand::rngs::ThreadRng;
use rand::seq::index;

use crate::qap_problem::QapProblem;
use crate::solution::Solution;
use crate::solvers::solver::Solver;
use crate::utils::*;
use rand::Rng;

pub struct HeuristicSolver<'a> {
    problem: &'a QapProblem,
    rng: ThreadRng,
    iter_count: i32,   // The number of times the LS loop is ran
    update_count: i32, // The number of times a solution is updated
}

impl<'a> HeuristicSolver<'a> {
    // Constructor
    pub fn new(problem: &'a QapProblem) -> HeuristicSolver<'a> {
        // Vector of pairs (i, j), moves in order
        let rng = rand::thread_rng();
        let iter_count: i32 = 0;
        let update_count: i32 = 0;
        HeuristicSolver {
            problem,
            rng,
            iter_count,
            update_count,
        }
    }

    // This greedy algorithm is called "Min Flow to Max Distance" heuristic
    // The idea is to always select the facility that the row sum is the smallest
    // and assign it to the location that the row sum is the largest.
    // The idea is taken from this video: https://www.youtube.com/watch?v=hZgS-iV-6Mk&ab_channel=softwaround%27me
    fn solve_heuristic(&mut self) -> Solution {
        let n = self.problem.get_n();
        // initialize empty solution
        let mut solution_array = vec![0; n];

        // create an array with row sums of the distance matrix
        let mut dist_sums = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                dist_sums[i] += self.problem.matrix_a_ref()[i][j];
            }
        }
        // create an array with row sums of the flow matrix
        let mut fac_sums = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                fac_sums[i] += self.problem.matrix_b_ref()[i][j];
            }
        }

        // draw a random facility to start with
        let random_facility = self.rng.gen_range(0..n);
        // draw a random location to start with
        let random_location = self.rng.gen_range(0..n);
        // assign the facility to the location
        solution_array[random_location] = random_facility;

        // for the selected random_facility change it
        // value in fac_sums to the highest possibly (to not select it again)
        fac_sums[random_facility] = std::i32::MAX;

        // for the selected random_location change it
        // value in dist_sums to the lowest possibly (to not select it again)
        dist_sums[random_location] = std::i32::MIN;

        // fill the rest of the solution
        for i in 1..n {
            // take index of the minimum value in the fac_sums
            let (min_fac_index, _value) = fac_sums.iter().enumerate().min_by_key(|&(_idx, &val)| val).unwrap_or((0, &0));
            // take index of the maximum value in the dist_sums
            let (max_dist_index, _value) = dist_sums.iter().enumerate().max_by_key(|&(_idx, &val)| val).unwrap_or((0, &0));
            solution_array[max_dist_index] = min_fac_index;
            fac_sums[min_fac_index] = std::i32::MAX;
            dist_sums[max_dist_index] = std::i32::MIN;
        }

        let mut solution = Solution::new(solution_array);
        solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        solution

    }

    
    fn solve_heuristic_min_flow(&mut self) -> Solution {
        let n = self.problem.get_n();
        // initialize empty solution
        let mut solution_array = vec![0; n];
        // draw a random facility to start with
        let random_facility = self.rng.gen_range(0..n);
        // assign the facility to the first location
        solution_array[0] = random_facility;

        // create a mutable copy of the flow matrix
        let mut flow_matrix = self.problem.matrix_b_ref().clone();

        // loop over the rest of the locations
        for i in 1..n {
            // previous facility
            let prev_facility = solution_array[i - 1];
            // iterate over cost flows for this facility and select the one with the smallest cost
            let (min_fac_index, _value) = flow_matrix[prev_facility].iter().enumerate().min_by_key(|&(_idx, &val)| val).unwrap_or((0, &0));
            
            // assign the facility with the smallest cost to the location
            solution_array[i] = min_fac_index;
            // change the flow cost of [prev_facility][min_cost_index] to the maximum value and [min_cost_index][prev_facility] to the maximum value
            // to avoid selecting the same facility again
            for j in 0..n {
                flow_matrix[prev_facility][j] = std::i32::MAX;
                flow_matrix[j][min_fac_index] = std::i32::MAX;
            }
        }
        let mut solution = Solution::new(solution_array);
        solution.evaluate(self.problem.matrix_a_ref(), self.problem.matrix_b_ref());
        solution
    }
}

impl<'a> Solver for HeuristicSolver<'a> {
    // Just greedy
    fn solve(&mut self) -> Solution {
        self.solve_heuristic()
        // TODO: move generate random solution to QAP problem class
        
    }
    fn get_iter_count(&self) -> i32 {
        self.iter_count
    }
    fn get_update_count(&self) -> i32 {
        0
    }
    fn get_initial_solution(&self) -> Option<Solution> {
        None
    }
}

use rand::rngs::ThreadRng;

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

    // Generate the solution using the algorithm similar
    // to the greedy cycle algorithm for the TSP problem
    // It's not deterministic, as we always by random
    // choose the best facility from the two best ones
    // that should be inserted at a given location
    fn solve_heuristic(&mut self) -> Solution {
        let n = self.problem.get_n();
        // initialize empty solution
        let mut solution_array = vec![0; n];

        // assign a random facility to the first location
        solution_array[0] = self.rng.gen_range(0..n);

        // declare a hashmap for storing used facilities
        let mut used_facilities = std::collections::HashSet::new();
        used_facilities.insert(solution_array[0]);
        let mut iter_count = 0;

        for i in 1..n {
            // we will choose by random from two the best facilities that minimized the cost
            let mut min_cost = std::i32::MAX;
            let mut second_min_cost = std::i32::MAX;
            let mut min_cost_facility = 0;
            let mut second_min_cost_facility = 0;

            for j in 0..n {
                if !used_facilities.contains(&j) {
                    let facility1 = solution_array[j];
                    let location1 = j;
                    let mut cost = 0;
                    for k in 0..i {
                        let facility2 = solution_array[k];
                        let location2 = k;
                        cost += self.problem.matrix_a_ref()[location1][location2] * self.problem.matrix_b_ref()[facility1][facility2];
                    }
                    if cost < second_min_cost {
                        second_min_cost = cost;
                        second_min_cost_facility = j;
                        if cost < min_cost {
                            second_min_cost = min_cost;
                            second_min_cost_facility = min_cost_facility;
                            min_cost = cost;
                            min_cost_facility = j;
                        }
                    }
                }
            }
            // Choose with 50% probability the best facility
            if self.rng.gen_range(0..2) == 0 {
                solution_array[i] = second_min_cost_facility;
                used_facilities.insert(second_min_cost_facility);
            } else {
                solution_array[i] = min_cost_facility;
                used_facilities.insert(min_cost_facility);
            }
            iter_count = iter_count + 1;
        }
        self.iter_count = iter_count;
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

#[derive(Debug)]
pub struct CandidateMove {
    pub pair: [usize; 2],
    pub delta: i32,
}

impl CandidateMove {
    pub fn new(pair: [usize; 2], delta: i32) -> CandidateMove{
        CandidateMove { pair, delta }
    }

    pub fn get_delta(&self) -> i32 {
        self.delta
    }
}

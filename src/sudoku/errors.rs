use std::*;

#[derive(Debug, Clone)]
pub struct SolveError;

impl fmt::Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "couldn't find a solution to given sukoku board")
    }
}

#[derive(Debug, Clone)]
pub struct LoadFromBytesError;

impl fmt::Display for LoadFromBytesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something went wrong when loading sudoku from file")
    }
}
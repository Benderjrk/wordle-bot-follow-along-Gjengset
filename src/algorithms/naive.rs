use crate::{Correctness, Guesser, Guess};

pub struct Naive;

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        todo!();
    }
}
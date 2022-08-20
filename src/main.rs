const GAMES: &str = include_str!("../answers.txt");

fn main() {
    
    println!("Hello, world!");
}

fn play<G: Guesser>(answer: &'static str, guesser: G) {
    // play six round where it invokes guesser each round
}

enum Correctness {
    // Green
    Correct,
    // Yellow
    Misplaced,
    // Gray
    Wrong,
}
struct Guess {
    word: String,
    mask: [Correctness; 5],
}

trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
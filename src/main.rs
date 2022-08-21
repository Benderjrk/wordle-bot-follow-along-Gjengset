const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = atlas::algorithms::Naive::new();
        atlas::play(answer, guesser);
    }
}
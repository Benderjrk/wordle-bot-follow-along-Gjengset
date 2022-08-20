const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let guesser = atlas::algorithms::Naive::new();
    for answer in GAMES.split_whitespace() {
        atlas::play(answer, guesser)
    }
}

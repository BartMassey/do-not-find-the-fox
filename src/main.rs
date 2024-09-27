//! "Do Not Find The Fox" two-player adversary search.
//!
//! https://donotfindthefox.com/

use hashbag::HashBag;

/// Letter tiles for the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    F,
    O,
    X,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    P1Win,
    P2Win,
    Draw,
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let outcome = match self {
            Outcome::P1Win => "Player 1 wins",
            Outcome::P2Win => "Player 2 wins",
            Outcome::Draw => "Draw",
        };
        write!(f, "{}", outcome)
    }
}

type Board = [[Option<Tile>; 4]; 4];

type Bag = HashBag<Tile>;

fn main() {
    let mut board: Board = Default::default();
    let mut bag: Bag = [(Tile::F, 5), (Tile::O, 6), (Tile::X, 5)]
        .into_iter()
        .collect();
    let outcome = play(&mut board, &mut bag);
    println!("{:?}", outcome);
}

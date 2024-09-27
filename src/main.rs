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

type Board = [[Option<Tile>; 4]; 4];

type Bag = HashBag<Tile>;

fn main() {
    let mut _board: Board = Default::default();
    let mut _bag: Bag = [(Tile::F, 5), (Tile::O, 6), (Tile::X, 5)]
        .into_iter()
        .collect();
    //let outcome = play(&mut board, &mut bag);
}

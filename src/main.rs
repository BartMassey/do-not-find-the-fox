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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    P1,
    P2,
}

impl Player {
    fn opponent(self) -> Self {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win(Player),
    Draw,
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let outcome = match self {
            Outcome::Win(Player::P1) => "Player 1 wins",
            Outcome::Win(Player::P2) => "Player 2 wins",
            Outcome::Draw => "Draw",
        };
        write!(f, "{}", outcome)
    }
}

type Board = [[Option<Tile>; 4]; 4];

type Bag = HashBag<Tile>;

fn foxed(board: &Board) -> bool {
    let fox_tiles = [Some(Tile::F), Some(Tile::O), Some(Tile::X)];
    let fox = |c| c == fox_tiles;

    for r0 in 0..4 {
        for c0 in 0..4 {
            if r0 <= 1 {
                let cand = [
                    board[r0 + 0][c0],
                    board[r0 + 1][c0],
                    board[r0 + 2][c0],
                ];
                if fox(cand) {
                    return true;
                }

                let cand = [
                    board[r0 + 2][c0],
                    board[r0 + 1][c0],
                    board[r0 + 0][c0],
                ];
                if fox(cand) {
                    return true;
                }
            }
            if c0 <= 1 {
                let cand = [
                    board[r0][c0 + 0],
                    board[r0][c0 + 1],
                    board[r0][c0 + 2],
                ];
                if fox(cand) {
                    return true;
                }

                let cand = [
                    board[r0][c0 + 2],
                    board[r0][c0 + 1],
                    board[r0][c0 + 0],
                ];
                if fox(cand) {
                    return true;
                }
            }

            if r0 <= 1 && c0 <= 1 {
                let cand = [
                    board[r0 + 0][c0 + 0],
                    board[r0 + 1][c0 + 1],
                    board[r0 + 2][c0 + 2],
                ];
                if fox(cand) {
                    return true;
                }

                let cand = [
                    board[r0 + 2][c0 + 2],
                    board[r0 + 1][c0 + 1],
                    board[r0 + 0][c0 + 0],
                ];
                if fox(cand) {
                    return true;
                }
            }

            if r0 >= 2 && c0 >= 2 {
                let cand = [
                    board[r0 - 0][c0 - 0],
                    board[r0 - 1][c0 - 1],
                    board[r0 - 2][c0 - 2],
                ];
                if fox(cand) {
                    return true;
                }

                let cand = [
                    board[r0 - 2][c0 - 2],
                    board[r0 - 1][c0 - 1],
                    board[r0 - 0][c0 - 0],
                ];
                if fox(cand) {
                    return true;
                }
            }
        }
    }

    false
}

fn play(on_move: Player, board: &mut Board, bag: &mut Bag) -> Outcome {
    if bag.is_empty() {
        return Outcome::Draw;
    }
    
    let opp = on_move.opponent();
    let mut result = Outcome::Win(opp);

    for r in 0..4 {
        for c in 0..4 {
            if board[r][c].is_some() {
                continue;
            }
            for p in [Tile::F, Tile::O, Tile::X] {
                if bag.contains(&p) == 0 {
                    continue;
                }
                bag.remove(&p);
                board[r][c] = Some(p);
                if !foxed(board) {
                    let outcome = play(opp, board, bag);
                    match outcome {
                        Outcome::Win(w) if w == on_move => {
                            board[r][c] = None;
                            bag.insert(p);
                            return outcome;
                        }
                        Outcome::Draw => {
                            result = Outcome::Draw;
                        }
                        _ => (),
                    }
                }
                board[r][c] = None;
                bag.insert(p);
            }
        }
    }
    result
}

fn main() {
    let mut board: Board = Default::default();
    let mut bag: Bag = [(Tile::F, 5), (Tile::O, 6), (Tile::X, 5)]
        .into_iter()
        .collect();
    assert_eq!(16, bag.len());
    let outcome = play(Player::P1, &mut board, &mut bag);
    println!("{:?}", outcome);
}

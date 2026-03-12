mod hand;
pub use hand::*;
mod set;
use itertools::Itertools;
use rand::seq::SliceRandom;
pub use set::*;
mod tile;
pub use tile::*;

fn set(s: &str) -> Set {
    todo!();
}

fn main() {
    let hand1 = hand::WinningHand {
        seat: Direction::South,
        sets: [
            Set::Gong([
                Tile::Characters(8),
                Tile::Characters(8),
                Tile::Characters(8),
                Tile::Characters(8),
            ]),
            Set::Pong([
                Tile::Characters(5),
                Tile::Characters(5),
                Tile::Characters(5),
            ]),
            Set::Pong([
                Tile::Characters(1),
                Tile::Characters(2),
                Tile::Characters(3),
            ]),
            Set::Pong([
                Tile::Characters(4),
                Tile::Characters(4),
                Tile::Characters(4),
            ]),
        ],
        pair: [Tile::Characters(9), Tile::Characters(9)],
    };

    let mut tiles = (4..=39)
        .chain(68..=103)
        .chain(132..=167)
        .chain(192..=219)
        .chain(224..=231)
        .collect::<Vec<u8>>();
    tiles.shuffle(&mut rand::rng());
    let hand = tiles.into_iter().take(14).sorted();

    for n in hand {
        if let Ok(t) = Tile::try_from(n) {
            println!("{n: >4} = {t}");
        } else {
            println!("-");
        }
    }

    //println!("{:?}", hand1.score());
}

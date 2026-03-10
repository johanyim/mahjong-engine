mod hand;
pub use hand::*;
mod set;
pub use set::*;
mod tile;
pub use tile::*;

fn set(s: &str) -> Set {
    todo!();
}

mod tests {
    #[test]
    fn set_testing() {}
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

    println!("{:?}", hand1.score());
}

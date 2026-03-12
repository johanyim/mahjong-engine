use std::str::FromStr;

use arrayvec::ArrayVec;

use super::*;

pub enum Direction {
    East = 0,
    South,
    West,
    North,
}
//struct SpecialHand {
//    tiles: [Tile; 14]
//}

/// a grouped representation of tiles without information of the game state
pub struct Hand {
    tiles: ArrayVec<Tile, 14>,
    sets: ArrayVec<Set, 4>,
    flowers: ArrayVec<Tile, 8>,
}

pub struct Player {
    wind: Direction,
    seat: Direction,
    hand: Hand,
}

pub struct Game {
    players: [Player; 4],
    wind: Direction,
    round: u32,
}

// drdgdw1c1c1c4c9s[1s1s1s](1s2s3s)f1f2f3f4f5f6f7f8
impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("defined mahjong notation parser")
    }
}

pub struct WinningHand {
    pub seat: Direction,
    pub sets: [Set; 4],
    pub pair: [Tile; 2],
}

impl WinningHand {
    /// 'fa' / 'hua'
    pub fn score(&self) -> (u16, Vec<&str>) {
        let mut scorers = vec![];

        // pong hand check
        match &self.sets {
            [Set::Gong(_), Set::Gong(_), Set::Gong(_), Set::Gong(_)] => scorers.push("gys"),
            sets if sets
                .into_iter()
                .all(|set| matches!(set, Set::Gong(_) | Set::Pong(_))) =>
            {
                scorers.push("ddw")
            }
            _ => (),
        }

        // sets + pair = ~14
        let mut tiles = self
            .sets
            .iter()
            .map(|x| x.into_iter())
            .flatten()
            .chain(self.pair.iter());

        if tiles.clone().all(Tile::is_honor) {
            scorers.push("big")
        };

        // wys check
        let mut numerics = tiles.clone().filter_map(Tile::suit);
        if let Some(first) = numerics.next()
            && numerics.all(|tile| tile == first)
        {
            if tiles.all(Tile::is_numeric) {
                scorers.push("qys")
            } else {
                scorers.push("wys")
            }
        }

        (3, scorers)
    }
}

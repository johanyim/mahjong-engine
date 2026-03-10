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

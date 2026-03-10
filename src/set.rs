use super::*;
// maybe unnecessary storage?
// might be better for type validation
pub enum Set {
    Pong([Tile; 3]),
    Gong([Tile; 4]),
    Seong([Tile; 3]),
    // // alternative
    //Pong(Tile, Tile, Tile),
    //Gong(Tile, Tile, Tile, Tile),
    //Seong(Tile, Tile, Tile),
}

pub struct SetIter {
    tiles: [Option<Tile>; 4],
    index: usize,
}

impl Iterator for SetIter {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < 4 {
            let i = self.index;
            self.index += 1;
            if let Some(t) = self.tiles[i].take() {
                return Some(t);
            }
        }
        None
    }
}

impl IntoIterator for Set {
    type Item = Tile;
    type IntoIter = SetIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Set::Pong([a, b, c]) => SetIter {
                tiles: [Some(a), Some(b), Some(c), None],
                index: 0,
            },
            Set::Seong([a, b, c]) => SetIter {
                tiles: [Some(a), Some(b), Some(c), None],
                index: 0,
            },
            Set::Gong([a, b, c, d]) => SetIter {
                tiles: [Some(a), Some(b), Some(c), Some(d)],
                index: 0,
            },
        }
    }
}

impl<'a> IntoIterator for &'a Set {
    type Item = &'a Tile;
    type IntoIter = std::slice::Iter<'a, Tile>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Set::Pong(x) => x.iter(),
            Set::Gong(x) => x.iter(),
            Set::Seong(x) => x.iter(),
        }
    }
}

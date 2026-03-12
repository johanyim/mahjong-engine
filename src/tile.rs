use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use itertools::Itertools;

// FUTURE: maybe there could be a way to design the tile notation to be lexicographically sortable
#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Bamboo(u8),     // b[1-9] 36
    Characters(u8), // c[1-9] 36
    Dots(u8),       // d[1-9] 36
    East,           // w0     4
    South,          // w1     4
    West,           // w2     4
    North,          // w3     4
    GreenDragon,    // zg     4
    RedDragon,      // zr     4
    WhiteDragon,    // zw     4

    // doesn't matter because they aren't included in the hand?
    FlowerA(u8), // F[0-3] 4 or potentially z?
    FlowerB(u8), // F[4-7] 4
}

#[derive(PartialEq, Eq)]
pub enum Suit {
    Bamboo,
    Characters,
    Dots,
}

impl Tile {
    pub fn is_honor(&self) -> bool {
        matches!(
            self,
            Tile::East
                | Tile::South
                | Tile::West
                | Tile::North
                | Tile::GreenDragon
                | Tile::RedDragon
                | Tile::WhiteDragon
        )
    }

    pub fn is_direction(&self) -> bool {
        matches!(self, Tile::East | Tile::South | Tile::West | Tile::North)
    }

    pub fn is_dragon(&self) -> bool {
        matches!(
            self,
            Tile::RedDragon | Tile::WhiteDragon | Tile::GreenDragon
        )
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Tile::Bamboo(_) | Tile::Dots(_) | Tile::Characters(_))
    }

    pub fn as_number(&self) -> Option<u8> {
        match self {
            Tile::Bamboo(n) | Tile::Dots(n) | Tile::Characters(n) if (1..=9).contains(n) => {
                Some(*n)
            }
            _ => None,
        }
    }

    pub fn suit(&self) -> Option<Suit> {
        match self {
            Tile::Bamboo(_) => Some(Suit::Bamboo),
            Tile::Dots(_) => Some(Suit::Dots),
            Tile::Characters(_) => Some(Suit::Characters),
            _ => None,
        }
    }
}

impl TryFrom<u8> for Tile {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // 1.: determine suit:
        //
        //[xx______]
        let suit_bits = value >> 6;
        //[__xxxx__]
        let ordinal_bits = (value >> 2) % (1 << 4);
        //[______xx]
        let identifier_bits = (value >> 0) % (1 << 2); // 4 bits
        //
        // 2.: numeric
        let check_number = move |n: u8| if (1..=9).contains(&n) { Ok(n) } else { Err(()) };
        assert_eq!(Ok(2), check_number(2));

        match suit_bits {
            // numeric
            0b00 => return Ok(Tile::Bamboo(check_number(ordinal_bits)?)),
            0b01 => return Ok(Tile::Characters(check_number(ordinal_bits)?)),
            0b10 => return Ok(Tile::Dots(check_number(ordinal_bits)?)),
            // honor/flower
            0b11 => (),
            // invalid
            _ => return Err(()), // ERROR: invalid state
        };

        // 3.: honor / flower
        match ordinal_bits {
            0b00_00 => Ok(Tile::East),
            0b00_01 => Ok(Tile::South),
            0b00_10 => Ok(Tile::West),
            0b00_11 => Ok(Tile::North),

            0b01_00 => Ok(Tile::GreenDragon),
            0b01_01 => Ok(Tile::RedDragon),
            0b01_10 => Ok(Tile::WhiteDragon),

            0b10_00 => Ok(Tile::FlowerA(identifier_bits)),
            0b10_01 => Ok(Tile::FlowerB(identifier_bits)),
            _ => Err(()), // ERROR: invalid state
        }
    }
}

impl Into<u8> for Tile {
    fn into(self) -> u8 {
        match self {
            Tile::Bamboo(n) => 0b00_0000_00 + (n << 2),
            Tile::Characters(n) => 0b01_0000_00 + (n << 2),
            Tile::Dots(n) => 0b10_0000_00 + (n << 2),
            Tile::East => 0b11_0000_00,
            Tile::South => 0b11_0001_00,
            Tile::West => 0b11_0010_00,
            Tile::North => 0b11_0011_00,
            Tile::GreenDragon => 0b11_0100_00,
            Tile::RedDragon => 0b11_0101_00,
            Tile::WhiteDragon => 0b11_0110_00,
            Tile::FlowerA(n) => 0b11_1000_00 + n,
            Tile::FlowerB(n) => 0b11_1001_00 + n,
        }
    }
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cc = s.chars();
        let first = cc.next();
        let second = cc.next();

        if let (Some(first), Some(second)) = (first, second) {
            //return Ok(Tile::Characters(2));
            todo!()
        }

        return Err(());
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c1 = match self {
            Tile::Bamboo(_) => 'b',
            Tile::Characters(_) => 'c',
            Tile::Dots(_) => 'd',
            Tile::East | Tile::South | Tile::West | Tile::North => 'w',
            Tile::GreenDragon | Tile::RedDragon | Tile::WhiteDragon => 'z',
            Tile::FlowerA(_) | Tile::FlowerB(_) => 'F',
        };

        let c2 = match self {
            Tile::Bamboo(n) | Tile::Characters(n) | Tile::Dots(n) => unsafe {
                char::from_u32_unchecked((*n + b'0') as u32)
            },
            Tile::East => '0',
            Tile::South => '1',
            Tile::West => '2',
            Tile::North => '3',
            Tile::GreenDragon => '0',
            Tile::RedDragon => '1',
            Tile::WhiteDragon => '2',

            Tile::FlowerA(n) => unsafe { char::from_u32_unchecked((*n + b'0') as u32) },
            Tile::FlowerB(n) => unsafe { char::from_u32_unchecked((*n + b'4') as u32) },
        };

        f.write_char(c1).and_then(|_| f.write_char(c2))
    }
}

mod tests {
    use super::*;

    #[test]
    fn tile_to_bits_conversion() {
        let sticks8_bits = 0b_10_1000_00u8;
        let sticks8_tile = Tile::try_from(0b_10_1000_00u8);
        assert_eq!(sticks8, Ok(Tile::Bamboo(8)));

        let red_dragon = Tile::try_from(0b_11_0111_01u8);
        assert_eq!(red_dragon, Ok(Tile::RedDragon));
    }

    #[test]
    fn parsing() {
        let tile = Tile::from_str("22");

        assert_eq!(tile, Ok(Tile::Characters(2)))
    }
}

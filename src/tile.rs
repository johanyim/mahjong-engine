use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub enum Tile {
    Sticks(u8),     // 36
    Circles(u8),    // 36
    Characters(u8), // 36
    East,           // 4
    South,          // 4
    West,           // 4
    North,          // 4
    RedDragon,      // 4
    WhiteDragon,    // 4
    GreenDragon,    // 4
    FlowerA(u8),    // 4
    FlowerB(u8),    // 4
}

impl Tile {
    pub fn is_honor(&self) -> bool {
        matches!(
            self,
            Tile::East
                | Tile::South
                | Tile::West
                | Tile::North
                | Tile::RedDragon
                | Tile::WhiteDragon
                | Tile::GreenDragon
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
        matches!(
            self,
            Tile::Sticks(_) | Tile::Circles(_) | Tile::Characters(_)
        )
    }

    pub fn as_number(&self) -> Option<u8> {
        match self {
            Tile::Sticks(n) | Tile::Circles(n) | Tile::Characters(n) if (1..=9).contains(n) => {
                Some(*n)
            }
            _ => None,
        }
    }

    pub fn suit(&self) -> Option<Suit> {
        match self {
            Tile::Sticks(_) => Some(Suit::Sticks),
            Tile::Circles(_) => Some(Suit::Circles),
            Tile::Characters(_) => Some(Suit::Characters),
            _ => None,
        }
    }
}

impl FromStr for Tile {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x=s.chars().chunks(2).into_iter().map(|s| {
            assert_eq!(s.len(), 2);
            let first = s.next().;
            let second = 

        });
        // TODO:
        todo!()
    }
}

#[derive(PartialEq, Eq)]
pub enum Suit {
    Sticks,
    Circles,
    Characters,
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

        //println!("suit =          {suit_bits:0>2b} ({suit_bits})");
        //println!("ordinal =     {ordinal_bits:0>4b} ({ordinal_bits})");
        //println!("identifier =    {identifier_bits:0>2b} ({identifier_bits})");

        //let reconstructed_bits =
        //    format!("{suit_bits:0>2b}{ordinal_bits:0>4b}{identifier_bits:0>2b}");
        //let raw_bits = format!("{value:0>8b}");
        //
        //assert_eq!(reconstructed_bits, raw_bits);

        // 2.: numeric
        let check_number = move |n: u8| if (1..=9).contains(&n) { Ok(n) } else { Err(()) };
        assert_eq!(Ok(2), check_number(2));

        match suit_bits {
            0b01 => return Ok(Tile::Sticks(check_number(ordinal_bits)?)),
            0b10 => return Ok(Tile::Circles(check_number(ordinal_bits)?)),
            0b11 => return Ok(Tile::Characters(check_number(ordinal_bits)?)),
            // honor/flower
            0b00 => (),
            // invalid
            _ => return Err(()), // ERROR: invalid state
        };

        // 3.: honor / flower
        match ordinal_bits {
            0b0000 => Ok(Tile::East),
            0b0001 => Ok(Tile::South),
            0b0010 => Ok(Tile::West),
            0b0011 => Ok(Tile::North),

            0b0100 => Ok(Tile::GreenDragon),
            0b0101 => Ok(Tile::WhiteDragon),
            0b0110 => Ok(Tile::RedDragon),

            0b1000 => Ok(Tile::FlowerA(identifier_bits)),
            0b1001 => Ok(Tile::FlowerB(identifier_bits)),
            _ => Err(()), // ERROR: invalid state
        }
    }
}

impl Into<u8> for Tile {
    fn into(self) -> u8 {
        todo!()
    }
}

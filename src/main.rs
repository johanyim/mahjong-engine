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
    Flower1(u8),    // 4
    Flower2(u8),    // 4
}

enum Set {
    Pong(Tile),
    Gong(Tile),
    Seung([Tile; 3]),
}

//impl Into<u8> for Tile {
//    fn into(self) -> u8 {
//
//    }
//}

impl TryFrom<u8> for Tile {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // 1.: determine suit:
        let suit_bits = value >> 6; // 2 bits
        let ordinal_bits = (value >> 2) % (1 << 2); // 4 bits
        let identifier_bits = value % (4); // 4 bits

        //println!("suit =          {suit_bits:0>2b} ({suit_bits})");
        //println!("ordinal =     {ordinal_bits:0>4b} ({ordinal_bits})");
        //println!("identifier =    {identifier_bits:0>2b} ({identifier_bits})");

        println!("{suit_bits:0>2b}{ordinal_bits:0>4b}{identifier_bits:0>2b}");
        return Err(());

        // 2.: numeric
        let check_number = |n| if (1..=9).contains(n) { Ok(n) } else { Err(()) };
        match suit_bits {
            0b01 => return Ok(Tile::Sticks(*check_number(&ordinal_bits)?)),
            0b10 => return Ok(Tile::Circles(*check_number(&ordinal_bits)?)),
            0b11 => return Ok(Tile::Characters(*check_number(&ordinal_bits)?)),
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
            0b0111 => Ok(Tile::Flower1(identifier_bits)),
            0b1000 => Ok(Tile::Flower2(identifier_bits)),
            _ => Err(()),
        }
    }
}

pub struct Game {
    // 144 tiles
    thrown: [Tile; 144],
}

fn main() {
    for n in 0u8..=255u8 {
        //Tile
        let tile = Tile::try_from(n);
        println!("{n:0>8b} = {tile:?}");
    }
}

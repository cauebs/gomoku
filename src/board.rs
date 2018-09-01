use failure;

use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use game::PlayerIndicator;

#[derive(Fail, Debug, PartialEq)]
enum Error {
    #[fail(display = "Cell is already occupied.")]
    OccupiedCell,
    #[fail(display = "Coordinates out of bounds.")]
    CoordinatesOutOfBounds,
}

type BoardArray = [[Option<PlayerIndicator>; 15]; 15];
pub type Coord = (usize, usize);

#[derive(Debug, Default, Clone)]
pub struct Board(BoardArray);

impl Board {
    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn make_move(
        &mut self,
        player: PlayerIndicator,
        coords: (usize, usize),
    ) -> Result<(), failure::Error> {
        if coords.0 >= 15 || coords.1 >= 15 {
            Err(Error::CoordinatesOutOfBounds)?;
        }

        if self[coords.0][coords.1].is_some() {
            Err(Error::OccupiedCell)?;
        }

        self[coords.0][coords.1] = Some(player);
        Ok(())
    }
}

impl Deref for Board {
    type Target = BoardArray;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut BoardArray {
        &mut self.0
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use game::PlayerIndicator::*;

        writeln!(f, "  {}", " _".repeat(self.width()))?;

        for (i, row) in self.iter().enumerate() {
            write!(f, "{:X} |", i)?;

            for cell in row {
                let mark = match cell {
                    Some(Player1) => '1',
                    Some(Player2) => '2',
                    None => '_',
                };
                write!(f, "{}|", mark)?;
            }

            writeln!(f)?;
        }

        write!(f, "  ")?;
        for (j, _column) in self.0[0].iter().enumerate() {
            write!(f, " {:X}", j)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

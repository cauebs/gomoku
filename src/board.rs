use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use game::PlayerIndicator;

type BoardArray = [[Option<PlayerIndicator>; 15]; 15];

#[derive(Debug, Default)]
pub struct Board(BoardArray);

impl Board {
    pub fn width(&self) -> usize {
        self[0].len()
    }

    pub fn height(&self) -> usize {
        self.len()
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

        writeln!(f, "  {}", " _".repeat(self[0].len()))?;

        for (i, row) in self.iter().enumerate() {
            write!(f, "{:X} |", i)?;

            for cell in row {
                let mark = match cell {
                    Some(Human) => "H",
                    Some(Bot) => "B",
                    None => "_",
                };
                write!(f, "{}|", mark)?;
            }

            writeln!(f)?;
        }

        write!(f, "  ")?;
        for (j, _column) in self[0].iter().enumerate() {
            write!(f, " {:X}", j)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

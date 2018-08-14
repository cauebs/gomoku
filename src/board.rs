use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use players::PlayerIndicator;

type BoardArray = [[Option<PlayerIndicator>; 7]; 6];

#[derive(Debug, Default)]
pub struct Board(BoardArray);

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
        writeln!(f, " _ _ _ _ _ _ _")?;

        for row in self.iter() {
            write!(f, "|")?;

            for cell in row {
                let mark = match cell {
                    Some(PlayerIndicator::P1) => "1",
                    Some(PlayerIndicator::P2) => "2",
                    None => "_",
                };
                write!(f, "{}|", mark)?;
            }

            writeln!(f)?;
        }

        writeln!(f, " 0 1 2 3 4 5 6")?;
        Ok(())
    }
}

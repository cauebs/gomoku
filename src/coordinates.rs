use failure;

use std::str::FromStr;

pub struct Coordinates(pub usize, pub usize);

#[derive(Fail, Debug)]
#[fail(display = "Failed to parse coordinates.")]
pub struct ParseCoordinatesError;

impl FromStr for Coordinates {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');
        let x = it.next().ok_or(ParseCoordinatesError)?.parse()?;
        let y = it.next().ok_or(ParseCoordinatesError)?.parse()?;
        Ok(Coordinates(x, y))
    }
}

impl Coordinates {
    pub fn from_hex_str(s: &str) -> Result<Self, failure::Error> {
        let mut it = s.split(',');

        let x = it.next().ok_or(ParseCoordinatesError)?;
        let x = usize::from_str_radix(x, 16)?;

        let y = it.next().ok_or(ParseCoordinatesError)?;
        let y = usize::from_str_radix(y, 16)?;

        Ok(Coordinates(x, y))
    }
}

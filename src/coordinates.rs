#![allow(dead_code)]
use failure;

#[derive(Fail, Debug)]
#[fail(display = "Failed to parse coordinates.")]
pub struct ParseCoordinatesError;

pub fn coordinates_from_str(s: &str) -> Result<(usize, usize), failure::Error> {
    let mut it = s.split(',');
    let x = it.next().ok_or(ParseCoordinatesError)?.parse()?;
    let y = it.next().ok_or(ParseCoordinatesError)?.parse()?;
    Ok((x, y))
}

pub fn coordinates_from_hex_str(s: &str) -> Result<(usize, usize), failure::Error> {
    let mut it = s.split(',');

    let x = it.next().ok_or(ParseCoordinatesError)?;
    let x = usize::from_str_radix(x, 16)?;

    let y = it.next().ok_or(ParseCoordinatesError)?;
    let y = usize::from_str_radix(y, 16)?;

    Ok((x, y))
}

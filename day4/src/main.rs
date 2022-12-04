use std::{path::Path, str::FromStr};

#[derive(Debug)]
pub enum Error {
    PairParseError(&'static str),
    IO(std::io::Error),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PairParseError(e) => f.write_fmt(format_args!("Failed to parse input {e}")),
            Error::IO(e) => f.write_fmt(format_args!("Failed due to IO {e}")),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

#[derive(Debug)]
pub struct Pair(u32, u32);

impl Pair {
    pub fn range(&self) -> std::ops::Range<u32> {
        self.0..(self.1 + 1)
    }
}

impl FromStr for Pair {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_part, second_part) = s
            .split_once('-')
            .ok_or(Error::PairParseError("Pair doesn't contain '-'"))?;
        let first_value = first_part
            .parse::<u32>()
            .map_err(|_| Error::PairParseError("Not a valid number"))?;
        let second_value = second_part
            .parse::<u32>()
            .map_err(|_| Error::PairParseError("Not a valid number"))?;

        Ok(Pair(first_value, second_value))
    }
}

fn read_input(path: impl AsRef<Path>, full_overlap: bool) -> Result<u32, Error> {
    let buff = std::fs::read_to_string(path)?;
    let lines = buff.lines();
    let mut duplicated_ones = 0;
    for line in lines {
        let (first_pair, second_pair) = line.trim().split_once(',').unwrap();
        let (first_pair, second_pair) = (Pair::from_str(first_pair)?, Pair::from_str(second_pair)?);
        if full_overlap {
            if first_pair.range().all(|e| second_pair.range().contains(&e))
                || second_pair.range().all(|e| first_pair.range().contains(&e))
            {
                duplicated_ones += 1;
            }
        } else if first_pair.range().any(|e| second_pair.range().contains(&e))
            || second_pair.range().any(|e| first_pair.range().contains(&e))
        {
            duplicated_ones += 1;
        }
    }
    Ok(duplicated_ones)
}

fn main() {
    match read_input("./src/input.txt", false) {
        Ok(duplicated_ones) => {
            println!("There are {duplicated_ones} duplicated efforts");
        }
        Err(err) => {
            println!("Failed to parse input {:#?}", err);
        }
    }
}

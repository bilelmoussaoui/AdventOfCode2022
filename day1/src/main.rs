use std::fs::File;
use std::{io::Read, path::Path};

pub type Calorie = u32;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Parse(std::num::ParseIntError),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => f.write_fmt(format_args!("Failed to parse the file due to I/O {err}")),
            Self::Parse(err) => f.write_fmt(format_args!("Failed to parse a number {err}")),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::Parse(e)
    }
}

#[derive(Debug, Default)]
pub struct Elf(Vec<Calorie>);

impl Elf {
    pub fn add_to_inventory(&mut self, calroie: Calorie) {
        self.0.push(calroie);
    }

    pub fn total_calories(&self) -> Calorie {
        self.0.iter().sum()
    }
}

fn read_input(input: impl AsRef<Path>) -> Result<Vec<Elf>, Error> {
    let mut f = File::open(input)?;
    let mut buf = String::default();
    f.read_to_string(&mut buf)?;
    let mut result = Vec::default();
    let mut elf = Elf::default();
    for line in buf.split('\n') {
        if line.is_empty() {
            result.push(elf);
            elf = Elf::default();
        } else {
            let calroie = line.trim().parse::<Calorie>()?;
            elf.add_to_inventory(calroie);
        }
    }
    Ok(result)
}

fn main() {
    match read_input("./src/input.txt") {
        Ok(mut elves) => {
            elves.sort_by_key(|elf| std::cmp::Reverse(elf.total_calories()));
            let best_elf = elves.get(0).unwrap();
            println!(
                "The elf carrying the most calroies has {} calories",
                best_elf.total_calories()
            );

            let total_three_best: Calorie =
                elves.iter().take(3).map(|elf| elf.total_calories()).sum();
            println!(
                "The sum of the three Elves carrying the most is {} calories",
                total_three_best
            );
        }
        Err(err) => {
            println!("Failed to parse file {err}");
        }
    };
}

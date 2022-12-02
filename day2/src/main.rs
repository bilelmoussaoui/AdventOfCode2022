use std::{cmp::Ordering, fs::File, io::Read, path::Path, str::FromStr};

#[derive(Debug)]
pub enum Error {
    InvalidShape(String),
    IO(std::io::Error),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidShape(shape) => f.write_fmt(format_args!("Invalid shape {shape}")),
            Self::IO(err) => f.write_fmt(format_args!("IO error {err}")),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => Ordering::Less,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Ordering::Greater,
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Shape {
    pub fn score(&self) -> u8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    /// Gets what you should play in order to win/lose/end in a draw
    pub fn what_to_play(&self, ordering: Ordering) -> Self {
        match (self, ordering) {
            (Self::Rock, Ordering::Less)
            | (Self::Paper, Ordering::Greater)
            | (Self::Scissors, Ordering::Equal) => Self::Scissors,
            (Self::Rock, Ordering::Equal)
            | (Self::Paper, Ordering::Less)
            | (Self::Scissors, Ordering::Greater) => Self::Rock,
            (Self::Rock, Ordering::Greater)
            | (Self::Paper, Ordering::Equal)
            | (Self::Scissors, Ordering::Less) => Self::Paper,
        }
    }
}

impl FromStr for Shape {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            shape => Err(Error::InvalidShape(shape.to_owned())),
        }
    }
}

/// The first shape is what the opponent played
#[derive(Debug)]
pub struct Round((Shape, Shape));
impl Round {
    pub fn new(opponent: Shape, you: Shape) -> Self {
        Self((opponent, you))
    }

    fn opponent(&self) -> &Shape {
        &self.0 .0
    }

    fn you(&self) -> &Shape {
        &self.0 .1
    }

    pub fn score(&self) -> u8 {
        match self.opponent().cmp(self.you()) {
            Ordering::Less => self.you().score() + 6,
            Ordering::Equal => self.you().score() + 3,
            Ordering::Greater => self.you().score(),
        }
    }
}

fn read_input(path: impl AsRef<Path>) -> Result<Vec<Round>, Error> {
    let mut f = File::open(path)?;
    let mut buff = String::default();
    f.read_to_string(&mut buff)?;

    let mut rounds = Vec::default();

    for line in buff.lines() {
        let mut words = line.split_whitespace();
        let shape1 = Shape::from_str(words.next().unwrap())?;
        // uncomment for the first puzzle
        // let shape2 = Shape::from_str(words.next().unwrap())?;
        // seconds puzzle
        let shape2 = match words.next().unwrap() {
            "X" => shape1.what_to_play(Ordering::Less),
            "Y" => shape1.what_to_play(Ordering::Equal),
            "Z" => shape1.what_to_play(Ordering::Greater),
            _ => unreachable!(),
        };
        let round = Round::new(shape1, shape2);
        rounds.push(round);
    }
    Ok(rounds)
}

fn main() {
    match read_input("./src/input.txt") {
        Ok(rounds) => {
            let total_score = rounds.iter().map(|round| round.score() as u32).sum::<u32>();
            println!("{}", total_score)
        }
        Err(err) => {
            println!("Failed to read input {err}")
        }
    }
}

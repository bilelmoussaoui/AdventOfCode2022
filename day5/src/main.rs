use std::{
    collections::{BTreeMap, VecDeque},
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
pub enum Error {
    ParseStepError(String),
}

#[derive(Debug, Clone)]
pub struct Crate(String);

impl std::fmt::Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug)]
pub struct Step {
    quantity: u32,
    from: usize,
    to: usize,
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("move") {
            return Err(Error::ParseStepError(
                "Step doesn't start with move".to_owned(),
            ));
        }

        let line = s.split_whitespace().collect::<Vec<_>>();
        let quantity = line.get(1).unwrap().parse::<u32>().unwrap();
        let from = line.get(3).unwrap().parse::<usize>().unwrap();
        let to = line.get(5).unwrap().parse::<usize>().unwrap();

        Ok(Self { quantity, from, to })
    }
}

fn parse_stacks(input: Vec<&'_ str>) -> BTreeMap<usize, VecDeque<Crate>> {
    let line_length = input.first().unwrap().len();
    let mut stacks: BTreeMap<usize, VecDeque<Crate>> = BTreeMap::new();

    for j in 0..input.len() {
        let mut i = 0;
        let mut column = 1;
        while i < line_length {
            let crate_ = &input.get(j).unwrap()[i..i + 3];
            if !crate_.trim().is_empty() {
                let crate_ = Crate(crate_.to_owned());
                let crate_clone = crate_.clone();
                stacks
                    .entry(column)
                    .and_modify(move |e| e.push_back(crate_clone))
                    .or_insert_with(|| {
                        let mut vec = VecDeque::new();
                        vec.push_back(crate_);
                        vec
                    });
            }
            column += 1;
            i += 4;
        }
    }
    stacks
}

fn read_input(
    path: impl AsRef<Path>,
    move_all: bool,
) -> Result<BTreeMap<usize, VecDeque<Crate>>, std::io::Error> {
    let buff = std::fs::read_to_string(path)?;
    let lines = buff.lines();

    let mut stacks_lines = Vec::default();
    let mut steps = Vec::default();
    for line in lines {
        if !line.is_empty() && !line.starts_with("move") {
            if !line.trim().starts_with('1') {
                stacks_lines.push(line);
            }
        } else if line.starts_with("move") {
            steps.push(Step::from_str(line).unwrap());
        }
    }
    let mut stacks = parse_stacks(stacks_lines);
    for step in steps {
        let mut i = 0;
        let mut move_crates = VecDeque::default();
        while i < step.quantity {
            let crate_ = stacks.get_mut(&step.from).unwrap().pop_front().unwrap();
            if move_all {
                move_crates.push_front(crate_);
            } else {
                stacks
                    .entry(step.to)
                    .and_modify(|entry| entry.push_front(crate_));
            }
            i += 1;
        }
        if move_all {
            while let Some(crate_) = move_crates.pop_front() {
                stacks
                    .entry(step.to)
                    .and_modify(|entry| entry.push_front(crate_));
            }
        }
    }
    Ok(stacks)
}

fn main() {
    let mut input = read_input("./src/input.txt", true).unwrap();
    for i in 0..(input.len() + 1) {
        if let Some(crate_) = input.get_mut(&i).and_then(|crates| crates.pop_front()) {
            print!("{} ", crate_);
        } else {
            print!("");
        }
    }
}

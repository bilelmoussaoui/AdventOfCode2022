use std::{fs::File, io::Read, path::Path};
static CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn second_problem(path: impl AsRef<Path>) -> Result<u32, std::io::Error> {
    let mut f = File::open(path)?;
    let mut buff = String::default();
    f.read_to_string(&mut buff)?;

    let mut lines = buff.lines();
    let mut total_priorities = 0;
    loop {
        let first_line = lines.next();
        if first_line.is_none() {
            break;
        }
        let first_line = first_line.unwrap();
        let second_line = lines.next().unwrap();
        let third_line = lines.next().unwrap();

        let mut common_letter = char::default();
        for c in first_line.chars() {
            if second_line.contains(c) && third_line.contains(c) {
                common_letter = c;
                break;
            }
        }
        let group_priority = CHARACTERS.find(common_letter).unwrap() as u32 + 1;
        total_priorities += group_priority;
    }
    Ok(total_priorities)
}

fn first_problem(path: impl AsRef<Path>) -> Result<u32, std::io::Error> {
    let mut f = File::open(path)?;
    let mut buff = String::default();
    f.read_to_string(&mut buff)?;

    let mut total_priorities = 0;
    for line in buff.lines() {
        let length = line.len();
        let (first_part, second_part): (Vec<_>, Vec<_>) = line
            .chars()
            .enumerate()
            .partition(|(index, _val)| index < &(length / 2));
        let first_part = first_part
            .into_iter()
            .map(|(_index, c)| c)
            .collect::<Vec<_>>();
        let second_part = second_part
            .into_iter()
            .map(|(_index, c)| c)
            .collect::<Vec<_>>();

        let mut common_char = char::default();
        for c in first_part.iter() {
            if second_part.contains(c) {
                common_char = *c;
                break;
            }
        }
        let ruckstack_priority = CHARACTERS.find(common_char).unwrap() as u32 + 1;
        total_priorities += ruckstack_priority;
    }
    Ok(total_priorities)
}

fn main() {
    match (
        first_problem("./src/input.txt"),
        second_problem("./src/input2.txt"),
    ) {
        (Ok(total_priorities), Ok(total_group_priorities)) => {
            println!("The sum of the priorities is {total_priorities}");
            println!("The sum of the priorities per group is {total_group_priorities}");
        }
        _ => {
            println!("Failed to read the file");
        }
    };
}

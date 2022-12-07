use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug)]
pub enum Error {
    CommandParseError(&'static str),
    ListOutputParseError(&'static str),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandParseError(e) => f.write_str(e),
            Self::ListOutputParseError(e) => f.write_str(e),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    CurrentDirectory(PathBuf),
    List,
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s.trim_start_matches('$').trim();
        if command.starts_with("ls") {
            Ok(Self::List)
        } else if command.starts_with("cd") {
            let path = command.trim_start_matches("cd").trim();
            Ok(Self::CurrentDirectory(path.into()))
        } else {
            Err(Error::CommandParseError("Failed to parse command"))
        }
    }
}

#[derive(Debug, Clone)]
pub enum ListOutput {
    Directory(PathBuf),
    File(u128, PathBuf),
}

impl FromStr for ListOutput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir") {
            let path = s.trim_start_matches("dir").trim();
            Ok(Self::Directory(path.into()))
        } else {
            let mut file = s.split_whitespace();
            let size = file.next().unwrap().parse::<u128>().unwrap();
            let path = file.next().unwrap();
            Ok(Self::File(size, path.into()))
        }
    }
}

/// Similar to fs::canonicalize but doesn't try to resolve the files
fn fake_canonicalize(path: impl AsRef<Path>) -> PathBuf {
    let mut new_path = PathBuf::default();
    for comp in path.as_ref().components() {
        new_path = match comp {
            std::path::Component::RootDir => new_path.join("/"),
            std::path::Component::ParentDir => new_path.parent().unwrap().into(),
            std::path::Component::Normal(p) => new_path.join(p),
            _ => unreachable!(),
        };
    }
    new_path
}

fn compute_size(tree: &HashMap<PathBuf, Vec<ListOutput>>, parent: impl AsRef<Path>) -> u128 {
    let mut total_size = 0;
    for v in tree.get(parent.as_ref()).unwrap() {
        match v {
            ListOutput::Directory(p) => {
                let new_path = parent.as_ref().join(p);
                total_size += compute_size(tree, new_path);
            }
            ListOutput::File(size, _) => {
                total_size += size;
            }
        }
    }
    total_size
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let buff = std::fs::read_to_string("./src/input.txt")?;
    let mut current_directory = PathBuf::default();
    let mut is_list = false;
    let mut tree: HashMap<PathBuf, Vec<ListOutput>> = HashMap::default();

    for line in buff.lines() {
        // This is a command
        if line.starts_with('$') {
            match Command::from_str(line)? {
                Command::CurrentDirectory(dir) => {
                    current_directory = fake_canonicalize(current_directory.join(dir));
                }
                Command::List => {
                    is_list = true;
                }
            }
        } else if is_list {
            let output = ListOutput::from_str(line)?;
            let output_clone = output.clone();
            tree.entry(current_directory.clone())
                .and_modify(|entries| entries.push(output))
                .or_insert_with(|| vec![output_clone]);
        }
    }

    let mut total_size = 0;
    for key in tree.keys() {
        let size = compute_size(&tree, key);
        if size <= 100000 {
            total_size += size;
        }
    }
    println!("Total size of files at most 100000: {total_size}");

    // Smallest directory to remove
    let required_storage = 70000000 - 30000000;
    let used_storage = compute_size(&tree, "/");
    println!("Used storage: {used_storage}");

    let needed_storage = used_storage - required_storage;
    println!("Needed storage: {needed_storage}");

    let mut smallest_directory = PathBuf::default();
    let mut smallest_size = u128::MAX;
    for key in tree.keys() {
        let size = compute_size(&tree, key);

        if size >= needed_storage && size <= smallest_size {
            smallest_size = size;
            smallest_directory = key.to_owned();
        }
    }
    println!(
        "Smallest directory at {:#?} with size {smallest_size}",
        smallest_directory
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::fake_canonicalize;
    #[test]
    fn canonicalize() {
        assert_eq!(fake_canonicalize("/a/e/../../d"), PathBuf::from("/d"));
    }
}

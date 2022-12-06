use std::collections::HashSet;

fn first_marker(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<_>>();
    let mut i = 3; // we start at the fourth letter
    let total_chars = chars.len();
    while i < total_chars {
        let last_four_chars: HashSet<char> = HashSet::from_iter(chars[(i - 3)..(i + 1)].to_owned());
        i += 1;
        if last_four_chars.len() == 4 {
            break;
        }
    }
    i
}

fn second_marker(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<_>>();
    let mut i = 13; // we start at the 13th letter
    let total_chars = chars.len();
    while i < total_chars {
        let last_fourten_chars: HashSet<char> =
            HashSet::from_iter(chars[(i - 13)..(i + 1)].to_owned());
        i += 1;
        if last_fourten_chars.len() == 14 {
            break;
        }
    }
    i
}

fn main() {
    let buff = std::fs::read_to_string("./src/input.txt").unwrap();
    let marker_1 = first_marker(&buff);
    println!("{}", marker_1);
    let marker_2 = second_marker(&buff);
    println!("{}", marker_2);
}

#[cfg(test)]
mod tests {
    use crate::{first_marker, second_marker};

    #[test]
    fn test_first_marker() {
        assert_eq!(first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(first_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_second_marker() {
        assert_eq!(second_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(second_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(second_marker("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(second_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(second_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}

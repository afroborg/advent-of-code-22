use std::{fs, str::Chars};

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn find_marker(chars: Chars, n: usize) -> usize {
    let mut count = 0;
    let offset = n - 1;

    for (i, c) in chars.clone().enumerate() {
        count += 1;

        if i < offset {
            continue;
        }

        let mut prev = chars
            .clone()
            .skip(i - offset)
            .take(offset)
            .collect::<Vec<_>>();

        prev.push(c);
        prev.sort();
        prev.dedup();

        if prev.len() == n {
            break;
        }
    }

    count
}

fn solve_part_1(data: &str) -> String {
    let split = data.chars();
    find_marker(split, 4).to_string()
}

fn solve_part_2(data: &str) -> String {
    let split = data.chars();
    find_marker(split, 14).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        assert_eq!(solve_part_1(data), "7");
    }

    #[test]
    fn test_part_2() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        assert_eq!(solve_part_2(data), "19");
    }
}

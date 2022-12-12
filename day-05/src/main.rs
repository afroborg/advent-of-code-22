#![allow(unused_must_use)]
use std::{collections::VecDeque, fs};

type Stack = VecDeque<char>;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn create_stacks(s: &str) -> Vec<Stack> {
    let mut lines: Vec<&str> = s.lines().collect();
    let nbr_of_stacks = lines.pop().unwrap().split("   ").count();
    let mut stacks: Vec<Stack> = vec![Stack::new(); nbr_of_stacks];

    lines.into_iter().for_each(|line| {
        let chars = line.chars().skip(1).step_by(4).collect::<Vec<_>>();

        chars.into_iter().enumerate().for_each(|(i, c)| {
            if !c.is_alphabetic() {
                return;
            }

            stacks[i].push_back(c);
        });
    });

    stacks
}

fn parse_step(step: &str) -> (usize, usize, usize) {
    let words = step.split(" ").collect::<Vec<_>>();

    let parse = |s: &str| s.parse::<usize>().unwrap();

    let nbr_of_moves = parse(words[1]);
    let from = parse(words[3]);
    let to = parse(words[5]);

    (nbr_of_moves, from, to)
}

fn solve_part_1(data: &str) -> String {
    let split_data = data.split("\n\n").collect::<Vec<_>>();
    let mut stacks = create_stacks(split_data[0]);

    let steps = split_data[1].lines().map(parse_step);

    steps.for_each(|(nbr_of_moves, from, to)| {
        for _ in 0..nbr_of_moves {
            let to_be_moved = stacks[from - 1].pop_front().unwrap();
            stacks[to - 1].push_front(to_be_moved);
        }
    });

    let result = stacks.into_iter().fold(String::new(), |mut acc, vec| {
        acc.push(vec[0]);
        acc
    });

    result.to_string()
}

fn solve_part_2(_data: &str) -> String {
    let split_data = _data.split("\n\n").collect::<Vec<_>>();
    let steps = split_data[1].lines().map(parse_step);
    let mut stacks = create_stacks(split_data[0]);

    steps.for_each(|(nbr_of_moves, from, to)| {
        let to_be_moved = stacks[from - 1].drain(0..nbr_of_moves).collect::<Vec<_>>();

        stacks[to - 1] = to_be_moved
            .into_iter()
            .chain(stacks[to - 1].drain(..))
            .collect();
    });

    let result = stacks.into_iter().fold(String::new(), |mut acc, vec| {
        acc.push(vec[0]);
        acc
    });

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(solve_part_1(data), "CMZ");
    }

    #[test]
    fn test_part_2() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        assert_eq!(solve_part_2(data), "MCD");
    }
}

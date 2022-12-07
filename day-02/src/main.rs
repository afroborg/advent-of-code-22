use std::{fs, str::FromStr};

#[derive(Debug)]
enum Score {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Score {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("Unknown move".to_string()),
        }
    }
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err("Unknown outcome".to_string()),
        }
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn solve_part_1(data: &str) -> String {
    let result = data
        .lines()
        .map(|line| {
            let moves = &line
                .split(" ")
                .map(|m| m.parse::<Score>().unwrap())
                .collect::<Vec<_>>()[0..2];

            use Outcome::*;
            use Score::*;
            match moves {
                [opponent, Rock] => {
                    let outcome = match opponent {
                        Paper => Lose,
                        Rock => Draw,
                        Scissors => Win,
                    };

                    outcome as i32 + Rock as i32
                }
                [opponent, Paper] => {
                    let outcome = match opponent {
                        Paper => Draw,
                        Rock => Win,
                        Scissors => Lose,
                    };

                    outcome as i32 + Paper as i32
                }
                [opponent, Scissors] => {
                    let outcome = match opponent {
                        Paper => Win,
                        Rock => Lose,
                        Scissors => Draw,
                    };

                    outcome as i32 + Scissors as i32
                }
                _ => 0,
            }
        })
        .sum::<i32>();

    result.to_string()
}

fn solve_part_2(data: &str) -> String {
    let result = data
        .lines()
        .map(|line| {
            let moves = line.split(" ").collect::<Vec<_>>();
            let elf_move = moves[0].parse::<Score>().unwrap();
            let outcome = moves[1].parse::<Outcome>().unwrap();

            use Outcome::*;
            use Score::*;

            let outcome_move = match outcome {
                Lose => match elf_move {
                    Rock => Scissors,
                    Scissors => Paper,
                    Paper => Rock,
                },
                Draw => elf_move,
                Win => match elf_move {
                    Rock => Paper,
                    Scissors => Rock,
                    Paper => Scissors,
                },
            };

            outcome_move as i32 + outcome as i32
        })
        .sum::<i32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::solve_part_1;

    #[test]
    fn it_works() {
        let data = "A Y
B X
C Z
";

        assert_eq!(solve_part_1(data), "15")
    }
}

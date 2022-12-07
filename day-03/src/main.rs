use std::{fs, str::FromStr};

struct Rucksack {
    compartment_one: Vec<char>,
    compartment_two: Vec<char>,
}

impl Rucksack {
    fn shared_items(&self) -> Vec<&char> {
        let mut shared_items = Vec::new();

        for item in self.compartment_one.iter() {
            if self.compartment_two.contains(&item) {
                shared_items.push(item);
            }
        }

        shared_items
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn solve_part_1(data: &str) -> String {
    let rucksacks = data.lines().map(|rucksack| {
        let items = rucksack.chars().collect::<Vec<_>>();
        let compartment_one = items[0..items.len() / 2].to_vec();
        let compartment_two = items[items.len() / 2..].to_vec();

        Rucksack {
            compartment_one,
            compartment_two,
        }
    });

    let priorities = rucksacks.map(|rucksack| -> i32 {
        rucksack
            .shared_items()
            .iter()
            .map(|c| {
                return match c {
                    'a'..='z' => **c as i32 - 96,
                    'A'..='Z' => **c as i32 - 38,
                    _ => 0,
                };
            })
            .max()
            .unwrap_or_default()
    });

    priorities.sum::<i32>().to_string()
}

fn solve_part_2(data: &str) -> String {
    todo!();
}

use std::{collections::HashSet, fs};

#[derive(Debug, Clone)]
struct Rucksack {
    all: Vec<char>,
    compartments: (Vec<char>, Vec<char>),
}

impl Rucksack {
    fn new(s: &str) -> Self {
        let items = s.chars().collect::<Vec<_>>();
        let size = items.len() / 2;

        let compartment_one = items[0..size].to_vec();
        let compartment_two = items[size..].to_vec();

        Rucksack {
            all: items,
            compartments: (compartment_one, compartment_two),
        }
    }

    fn shared_items(&self) -> Vec<&char> {
        let (compartment_one, compartment_two) = &self.compartments;

        let compartment_one: HashSet<_> = compartment_one.iter().collect();
        let compartment_two: HashSet<_> = compartment_two.iter().collect();

        compartment_one
            .intersection(&compartment_two)
            .copied()
            .collect()
    }

    fn to_priority_values(&self) -> Vec<i32> {
        self.all.iter().map(|c| map_to_value(c)).collect()
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn get_rucksacks(data: &str) -> Vec<Rucksack> {
    data.lines().map(Rucksack::new).collect()
}

fn map_to_value(c: &char) -> i32 {
    match c {
        'a'..='z' => *c as i32 - 96,
        'A'..='Z' => *c as i32 - 38,
        _ => 0,
    }
}

fn to_priority_values(rucksack: &Rucksack) -> Vec<i32> {
    rucksack
        .shared_items()
        .into_iter()
        .map(|c| map_to_value(c))
        .collect()
}

fn get_prioritized_value(rucksack: &Rucksack) -> i32 {
    to_priority_values(rucksack).iter().max().unwrap().clone()
}

fn solve_part_1(data: &str) -> String {
    let rucksacks = get_rucksacks(data);
    let priorities = rucksacks.iter().map(get_prioritized_value).sum::<i32>();

    priorities.to_string()
}

fn solve_part_2(data: &str) -> String {
    let rucksacks = get_rucksacks(data);
    let rucksack_groups = rucksacks.chunks(3);

    let group_values = rucksack_groups.map(|group| {
        let values = group.iter().flat_map(|rucksack| {
            let mut priority_vaules = rucksack.to_priority_values();

            priority_vaules.sort();
            priority_vaules.dedup();

            priority_vaules
        });

        let in_all_sacks = values
            .clone()
            .filter(move |v| values.clone().filter(|v2| v == v2).count() == 3);

        in_all_sacks.max().unwrap_or_default()
    });

    let total = group_values.sum::<i32>();
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(solve_part_1(data), "157");
    }

    #[test]
    fn test_part_2() {
        let data = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(solve_part_2(data), "70");
    }
}

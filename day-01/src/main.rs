use std::fs;

type Calorie = i32;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn solve_part_1(lines: &str) -> String {
    // get the total carry of each elf
    let carry = lines
        .split("\n\n") // get the elf data into its own vec
        .map(|elf_calories| {
            // get the calories for each elf
            elf_calories
                .lines()
                // convert the strings to number
                .map(|calorie| calorie.parse::<Calorie>().unwrap_or_default())
                // add all the lines
                .sum::<Calorie>()
        });

    // return the maximum number
    carry.max().unwrap_or_default().to_string()
}

fn solve_part_2(lines: &str) -> String {
    // get the total carry of each elf
    let mut carry = lines
        .split("\n\n") // get the elf data into its own vec
        .map(|elf_calories| {
            // get the calories for each elf
            elf_calories
                .lines()
                // convert the strings to number
                .map(|calorie| calorie.parse::<Calorie>().unwrap_or_default())
                // add all the lines
                .sum::<Calorie>()
        })
        .collect::<Vec<Calorie>>();

    carry.sort();
    carry.reverse();
    carry.iter().take(3).sum::<Calorie>().to_string()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), "24000");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), "45000");
    }
}

use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn solve_part_1(data: &str) -> String {
    todo!()
}

fn solve_part_2(data: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "";

        assert_eq!(solve_part_1(data), "");
    }

    #[test]
    fn test_part_2() {
        let data = "";

        assert_eq!(solve_part_2(data), "");
    }
}

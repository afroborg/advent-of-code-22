use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 1: {}", solve_part_2(&data));
}

fn solve_part_1(data: &str) -> String {
    todo!()
}

fn solve_part_2(data: &str) -> String {
    todo!();
}

use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn solve_part_1(data: &str) -> String {
    let lines = data
        .lines()
        .map(|line| {
            line.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows = lines.len();
    let columns = lines[0].len();

    // the the peritimer of the matrix
    let mut visible_trees = 2 * rows + 2 * columns - 4;

    for row in 1..rows - 1 {
        for column in 1..columns - 1 {
            let tree = lines[row][column];

            let mut above = lines.iter().take(row).map(|line| line[column]);
            let mut below = lines.iter().skip(row + 1).map(|line| line[column]);
            let left = lines[row][..column].to_vec();
            let right = lines[row][column + 1..].to_vec();

            let mut bigger_than_above = || above.all(|x| x < tree);
            let mut bigger_than_below = || below.all(|x| x < tree);
            let bigger_than_left = || left.iter().all(|x| x < &tree);
            let bigger_than_right = || right.iter().all(|x| x < &tree);

            if bigger_than_above()
                || bigger_than_below()
                || bigger_than_left()
                || bigger_than_right()
            {
                visible_trees += 1;
            }
        }
    }

    visible_trees.to_string()
}

fn solve_part_2(data: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "30373
25512
65332
33549
35390";

        assert_eq!(solve_part_1(data), "21");
    }

    #[test]
    fn test_part_2() {
        let data = "";

        assert_eq!(solve_part_2(data), "");
    }
}

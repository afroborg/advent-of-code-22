use std::{collections::HashSet, fs};

struct Move {
    direction: String,
    distance: i32,
}

impl Move {
    fn new(s: &str) -> Self {
        let mut split_move = s.split_whitespace();
        let direction = split_move.next().unwrap().to_string();
        let distance = split_move.next().unwrap().parse::<i32>().unwrap();

        Self {
            direction,
            distance,
        }
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn rope_movements(rope_size: usize, moves: Vec<Move>) -> usize {
    let starting_point = (0, 0);

    let mut rope = vec![starting_point.clone(); rope_size];
    let mut visited = HashSet::from([starting_point]);

    moves.iter().for_each(|m| {
        for _ in 0..m.distance {
            match m.direction.as_str() {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                _ => panic!("Invalid direction"),
            }

            for i in 0..rope_size - 1 {
                let head = rope[i];
                let tail = &mut rope[i + 1];

                let diff_x: i32 = head.0 - tail.0;
                let diff_y: i32 = head.1 - tail.1;

                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    let sign_x = diff_x.signum();
                    let sign_y = diff_y.signum();

                    // if we are moving horizontally
                    if diff_x == 0 {
                        tail.1 += sign_y;
                    // if we are moving vertically
                    } else if diff_y == 0 {
                        tail.0 += sign_x;
                    // if we are moving diagonally
                    } else {
                        tail.0 += sign_x;
                        tail.1 += sign_y;
                    }
                }
            }

            visited.insert(rope.last().unwrap().clone());
        }
    });

    visited.len()
}

fn solve_part_1(data: &str) -> String {
    let moves = data.lines();
    let visited = rope_movements(2, moves.map(Move::new).collect());

    visited.to_string()
}

fn solve_part_2(data: &str) -> String {
    let moves = data.lines();
    let visited = rope_movements(10, moves.map(Move::new).collect());

    visited.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(solve_part_1(data), "13");
    }

    #[test]
    fn test_part_2() {
        let data = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(solve_part_2(data), "36");
    }
}

use std::{
    collections::{HashSet, VecDeque},
    fs,
    ops::Sub,
};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Elevation {
    Start,
    End,
    Value(i32),
}

impl Elevation {
    fn new(c: char) -> Self {
        match c {
            'S' => Self::Start,
            'E' => Self::End,
            _ => Self::Value(c as i32 - 97),
        }
    }
}

type Location = (i32, i32);
type Grid = Vec<Vec<Elevation>>;

impl Sub for Elevation {
    type Output = i32;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a as i32 - b as i32,
            _ => panic!("Invalid subtraction"),
        }
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn parse_grid(data: &str) -> Grid {
    data.lines()
        .map(|line| line.chars().map(Elevation::new).collect())
        .collect()
}

fn replace_start_end(grid: &mut Grid) -> (Location, Location) {
    let mut start: Location = (0, 0);
    let mut end: Location = (0, 0);

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let elevation = grid[r][c];

            if elevation == Elevation::Start {
                start = (r as i32, c as i32);
                grid[r][c] = Elevation::Value(0);
            } else if elevation == Elevation::End {
                end = (r as i32, c as i32);
                grid[r][c] = Elevation::Value(25);
            }
        }
    }

    (start, end)
}

fn loop_queue<F1, F2>(start: Location, grid: Grid, altitude_check: F1, is_done: F2) -> i32
where
    F1: Fn(i32) -> bool,
    F2: Fn(Location) -> bool,
{
    let mut queue = VecDeque::from([(0_i32, start.0, start.1)]);
    let mut visited = HashSet::from([start]);

    let nbr_of_rows = grid.len() as i32;
    let nbr_of_columns = grid[0].len() as i32;

    while let Some((value, r, c)) = queue.pop_front() {
        for (next_row, next_column) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
            if next_row < 0
                || next_row >= nbr_of_rows
                || next_column >= nbr_of_columns
                || next_column < 0
            {
                continue;
            }

            if visited.contains(&(next_row, next_column)) {
                continue;
            }

            if altitude_check(
                grid[next_row as usize][next_column as usize] - grid[r as usize][c as usize],
            ) {
                continue;
            }

            if is_done((next_row, next_column)) {
                return value + 1;
            }

            visited.insert((next_row, next_column));
            queue.push_back((value + 1, next_row, next_column));
        }
    }

    0
}

fn solve_part_1(data: &str) -> String {
    let mut grid: Grid = parse_grid(data);

    let (start, end) = replace_start_end(&mut grid);
    let result = loop_queue(
        start,
        grid,
        |altitude| altitude > 1,
        |location| location == end,
    );

    result.to_string()
}

fn solve_part_2(data: &str) -> String {
    let mut grid: Grid = parse_grid(data);

    let (_, end) = replace_start_end(&mut grid);

    let result = loop_queue(
        end,
        grid.clone(),
        |altitude| altitude < -1,
        |location| grid[location.0 as usize][location.1 as usize] == Elevation::Value(0),
    );

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), "31");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), "29");
    }
}

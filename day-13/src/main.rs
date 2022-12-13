use serde::Deserialize;
use std::{cmp::Ordering, fs};

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Packet {
    Value(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn new(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }

    fn single_value_list(val: usize) -> Self {
        Packet::List(vec![Packet::Value(val)])
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Value(a), List(_)) => Packet::single_value_list(*a).cmp(other),
            (List(_), Value(b)) => self.cmp(&Packet::single_value_list(*b)),
            (Value(a), Value(b)) => a.cmp(b),
            (List(a), List(b)) => {
                for i in 0..a.len().min(b.len()) {
                    let order = a[i].cmp(&b[i]);

                    if order == Ordering::Equal {
                        continue;
                    }

                    return order;
                }

                a.len().cmp(&b.len())
            }
        }
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn parse_pairs(data: &str) -> Vec<(Packet, Packet)> {
    let pairs = data
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair.split_once("\n").unwrap();
            let left: Packet = Packet::new(left);
            let right: Packet = Packet::new(right);

            (left, right)
        })
        .collect();

    pairs
}

fn solve_part_1(data: &str) -> String {
    let pairs = parse_pairs(data);

    pairs
        .iter()
        .enumerate()
        .fold(0, |mut acc, (i, pair)| {
            let (left, right) = pair;

            if left < right {
                acc += i + 1;
            }

            acc
        })
        .to_string()
}

fn solve_part_2(data: &str) -> String {
    let list_list_val = |val: usize| Packet::List(vec![Packet::List(vec![Packet::Value(val)])]);
    let divider_packets = [list_list_val(2), list_list_val(6)];

    let mut pairs = data
        .lines()
        .filter(|l| !l.is_empty())
        .map(Packet::new)
        .chain(divider_packets.clone())
        .collect::<Vec<_>>();

    pairs.sort();

    pairs
        .iter()
        .enumerate()
        .fold(1, |mut acc, (i, packet)| {
            if divider_packets.contains(packet) {
                acc *= i + 1;
            }

            acc
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), "13");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), "140");
    }
}

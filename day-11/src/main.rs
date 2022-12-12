use std::{collections::VecDeque, fs};

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    touches: u64,
}

impl Monkey {
    fn from_chunk(chunk: &str) -> Self {
        let mut lines = chunk.lines().skip(1);

        let items = lines
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let operation = Operation::new(&lines.next().unwrap().split("= ").last().unwrap());

        let mut get_next_number = || -> u64 {
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        };

        let divisible = get_next_number();
        let value_one = get_next_number();
        let value_two = get_next_number();

        Self {
            items,
            operation,
            touches: 0,
            test: Test {
                condition: divisible,
                if_true: value_one,
                if_false: value_two,
            },
        }
    }

    fn inspect(&mut self, old: u64, common_denominator: u64) -> u64 {
        self.touches += 1;
        self.operation.inspect(old, common_denominator)
    }
}

#[derive(Debug, Clone)]
struct Test {
    condition: u64,
    if_true: u64,
    if_false: u64,
}

#[derive(Debug, Clone)]
struct Operation {
    value_one: Value,
    value_two: Value,
    operation: OperationType,
}

impl Operation {
    fn new(s: &str) -> Self {
        let mut split = s.trim().split(" ");

        let get_value = |s: &str| {
            if s == "old" {
                Value::This
            } else {
                Value::Other(s.parse().unwrap())
            }
        };

        let value_one = get_value(split.next().unwrap().trim());

        let operation = match split.next().unwrap() {
            "+" => OperationType::Add,
            "*" => OperationType::Multiply,
            _ => panic!("Unknown operation"),
        };

        let value_two = get_value(split.next().unwrap().trim());

        Self {
            value_one,
            value_two,
            operation,
        }
    }

    fn inspect(&self, old: u64, common_denominator: u64) -> u64 {
        let value_one = match self.value_one {
            Value::This => old,
            Value::Other(v) => v,
        };

        let value_two = match self.value_two {
            Value::This => old,
            Value::Other(v) => v,
        };

        let result = match self.operation {
            OperationType::Add => value_one + value_two,
            OperationType::Multiply => value_one * value_two,
        };

        result % common_denominator
    }
}

#[derive(Debug, Clone)]
enum OperationType {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
enum Value {
    This,
    Other(u64),
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn process(monkeys: &mut Vec<Monkey>, rounds: u64, divisor: u64) -> Vec<u64> {
    let common_denoniator = monkeys.iter().map(|m| m.test.condition).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let new_item = monkeys[i].inspect(item, common_denoniator);
                let new_item = new_item / divisor;

                let next_monkey = if new_item % monkeys[i].test.condition == 0 {
                    monkeys[i].test.if_true
                } else {
                    monkeys[i].test.if_false
                };

                monkeys[next_monkey as usize].items.push_back(new_item);
            }
        }
    }

    let mut touches = monkeys.iter().map(|m| m.touches).collect::<Vec<_>>();
    touches.sort();

    touches
}

fn solve_part_1(data: &str) -> String {
    let monkey_chunks = data.split("\n\n").map(|s| s.trim());

    let mut monkeys = monkey_chunks.map(Monkey::from_chunk).collect::<Vec<_>>();
    let touches = process(&mut monkeys, 20, 3);

    let result: u64 = touches.iter().rev().take(2).product();

    result.to_string()
}

fn solve_part_2(data: &str) -> String {
    let monkey_chunks = data.split("\n\n").map(|s| s.trim());

    let mut monkeys = monkey_chunks.map(Monkey::from_chunk).collect::<Vec<_>>();
    let touches = process(&mut monkeys, 10_000, 1);

    let result: u64 = touches.iter().rev().take(2).product();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), "10605");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), "2713310158");
    }
}

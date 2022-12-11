use std::{collections::VecDeque, fs};

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: Test,
    touches: usize,
}

impl Monkey {
    fn from_chunk(chunk: &str) -> Self {
        let mut lines = chunk.lines().skip(1);

        let items = lines
            .next()
            .unwrap()
            .replace("Starting items: ", "")
            .split(", ")
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let operation = Operation::new(&lines.next().unwrap().replace("Operation: new = ", ""));

        let divisible = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let value_one = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let value_two = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

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

    fn inspect(&mut self, old: usize) -> usize {
        self.touches += 1;
        self.operation.inspect(old)
    }
}

#[derive(Debug, Clone)]
struct Test {
    condition: usize,
    if_true: usize,
    if_false: usize,
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

    fn inspect(&self, old: usize) -> usize {
        let value_one = match self.value_one {
            Value::This => old,
            Value::Other(v) => v,
        };

        let value_two = match self.value_two {
            Value::This => old,
            Value::Other(v) => v,
        };

        match self.operation {
            OperationType::Add => value_one + value_two,
            OperationType::Multiply => value_one * value_two,
        }
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
    Other(usize),
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn solve_part_1(data: &str) -> String {
    let monkey_chunks = data.split("\n\n").map(|s| s.trim());

    let mut monkeys = monkey_chunks.map(Monkey::from_chunk).collect::<Vec<_>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let new_item = monkeys[i].inspect(item);
                let new_item = new_item / 3;

                let next_monkey = if new_item % monkeys[i].test.condition == 0 {
                    monkeys[i].test.if_true
                } else {
                    monkeys[i].test.if_false
                };

                monkeys[next_monkey].items.push_back(new_item);
            }
        }
    }

    let mut touches = monkeys.iter().map(|m| m.touches).collect::<Vec<_>>();
    touches.sort();

    let result = touches.pop().unwrap() * touches.pop().unwrap();

    result.to_string()
}

fn solve_part_2(data: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "Monkey 0:
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

        assert_eq!(solve_part_1(data), "10605");
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let data = "";

        assert_eq!(solve_part_2(data), "");
    }
}

use std::fs;

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    cycles: usize,
}

#[derive(Debug)]
enum Operation {
    Noop,
    Addx(i32),
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("{}", solve_part_2(&data));
}

fn parse_instructions(data: &str) -> Vec<Instruction> {
    let lines = data.lines();
    let mut instructions = Vec::new();

    lines.for_each(|line| {
        let split_line = line.split_whitespace().collect::<Vec<_>>();
        let operation = split_line[0];

        let instruction = match operation {
            "noop" => Instruction {
                operation: Operation::Noop,
                cycles: 1,
            },
            "addx" => {
                let value = split_line[1].parse::<i32>().unwrap();
                Instruction {
                    operation: Operation::Addx(value),
                    cycles: 2,
                }
            }
            _ => panic!("Invalid operation"),
        };

        instructions.push(instruction);
    });

    instructions
}

fn cycle(instructions: Vec<Instruction>, padd: bool) -> Vec<i32> {
    let mut register = 1;
    let mut cycles = vec![];

    if padd {
        cycles.push(register);
    }

    instructions.iter().for_each(|instruction| {
        for i in 0..instruction.cycles {
            match instruction.operation {
                Operation::Noop => cycles.push(register),
                Operation::Addx(value) => {
                    cycles.push(register);

                    if i == instruction.cycles - 1 {
                        register += value;
                    }
                }
            }
        }
    });

    cycles
}

fn solve_part_1(data: &str) -> String {
    let instructions = parse_instructions(data);
    let cycles = cycle(instructions, true);

    let sum: i32 = (20..=220).step_by(40).map(|i| cycles[i] * i as i32).sum();

    sum.to_string()
}

fn solve_part_2(data: &str) -> String {
    let instructions = parse_instructions(data);
    let cycles = cycle(instructions, false);

    let mut str = "".to_string();

    for i in (0..cycles.len()).step_by(40) {
        if i != 0 {
            str.push('\n');
        }

        for j in 0..40 {
            if (cycles[i + j] - j as i32).abs() <= 1 {
                str.push('#');
            } else {
                str.push('.');
            }
        }
    }

    str
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), "13140");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            solve_part_2(INPUT),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}

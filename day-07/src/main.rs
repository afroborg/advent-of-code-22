use std::collections::HashMap;
use std::fs;
use std::str::Lines;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn parse_commands(lines: Lines) -> HashMap<String, u32> {
    let mut context: Vec<&str> = vec![];
    let mut directories: HashMap<String, u32> = HashMap::new();

    lines.for_each(|line| {
        let words = line.split_whitespace().collect::<Vec<_>>();

        match words[1] {
            "cd" => {
                let dir_name = words[2];

                match dir_name {
                    "/" => {
                        context.push("");
                    }
                    ".." => {
                        context.pop();
                    }
                    _ => {
                        context.push(dir_name);
                    }
                }
            }
            "ls" => {
                return;
            }
            _ => {
                // this is a file
                if let Ok(size) = words[0].parse::<u32>() {
                    for i in 1..context.len() + 1 {
                        let path = context[0..i].join("/");
                        let files = directories.entry(path).or_insert(0);
                        *files += size;
                    }
                }
            }
        }
    });

    directories
}

fn solve_part_1(data: &str) -> String {
    let lines = data.lines();
    let directories = parse_commands(lines);

    let result = directories.into_iter().fold(0, |mut acc, (_, value)| {
        acc += if value <= 100000 { value } else { 0 };
        acc
    });

    result.to_string()
}

fn solve_part_2(data: &str) -> String {
    let lines = data.lines();

    let total_space = 70_000_000;
    let needed_space = 30_000_000;
    let max_usage = total_space - needed_space;

    let directories = parse_commands(lines);
    let root_size = directories.get("").unwrap();

    let need_to_free = root_size - max_usage;

    let dir_to_remove = directories
        .into_values()
        .filter(|value| value >= &need_to_free)
        .min()
        .unwrap();

    dir_to_remove.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(solve_part_1(data), "95437");
    }

    #[test]
    fn test_part_2() {
        let data = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(solve_part_2(data), "24933642");
    }
}

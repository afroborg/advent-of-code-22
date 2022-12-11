use std::fs;

struct TreeMatrix {
    trees: Vec<Vec<Tree>>,
    columns: usize,
    rows: usize,
}

impl TreeMatrix {
    fn new(s: &str) -> Self {
        let trees: Vec<Vec<Tree>> = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(column, value)| {
                        let size = value.to_digit(10).unwrap() as usize;

                        Tree { size, column, row }
                    })
                    .collect()
            })
            .collect();

        let rows = trees.len();
        let columns = trees[0].len();

        Self {
            trees,
            rows,
            columns,
        }
    }

    fn get_visible_trees(&self) -> Vec<Tree> {
        let mut visible_trees = vec![];

        for row in 1..self.rows - 1 {
            for column in 1..self.columns - 1 {
                let tree = self.get(row, column);

                if self.is_visible(tree) {
                    visible_trees.push(tree.clone());
                }
            }
        }

        visible_trees
    }

    fn get(&self, row: usize, column: usize) -> &Tree {
        &self.trees[row][column]
    }

    fn trees_above(&self, tree: &Tree) -> Vec<&Tree> {
        self.trees
            .iter()
            .take(tree.row)
            .map(|line| &line[tree.column])
            .rev()
            .collect()
    }

    fn trees_below(&self, tree: &Tree) -> Vec<&Tree> {
        self.trees
            .iter()
            .skip(tree.row + 1)
            .map(|line| &line[tree.column])
            .collect()
    }

    fn trees_left(&self, tree: &Tree) -> Vec<&Tree> {
        self.trees[tree.row][..tree.column].iter().rev().collect()
    }

    fn trees_right(&self, tree: &Tree) -> Vec<&Tree> {
        self.trees[tree.row][tree.column + 1..].iter().collect()
    }

    fn is_visible(&self, tree: &Tree) -> bool {
        let above = self.trees_above(tree);
        let below = self.trees_below(tree);
        let left = self.trees_left(tree);
        let right = self.trees_right(tree);

        let bigger_than_above = || above.into_iter().all(|x| x < tree);
        let bigger_than_below = || below.into_iter().all(|x| x < tree);
        let bigger_than_left = || left.into_iter().all(|x| x < &tree);
        let bigger_than_right = || right.into_iter().all(|x| x < &tree);

        bigger_than_above() || bigger_than_below() || bigger_than_left() || bigger_than_right()
    }

    fn calculate_scenic_score(&self, tree: &Tree) -> usize {
        let above = self.trees_above(tree);
        let below = self.trees_below(tree);
        let left = self.trees_left(tree);
        let right = self.trees_right(tree);

        let score = tree.scenic_score(above)
            * tree.scenic_score(below)
            * tree.scenic_score(left)
            * tree.scenic_score(right);

        score
    }

    fn get_best_scenic_score(&self) -> usize {
        self.get_visible_trees()
            .iter()
            .map(|tree| self.calculate_scenic_score(tree))
            .max()
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct Tree {
    size: usize,
    column: usize,
    row: usize,
}

impl Tree {
    fn scenic_score(&self, others: Vec<&Self>) -> usize {
        let mut val = 0;

        for other in others {
            val += 1;

            if other >= &self {
                break;
            }
        }

        val
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.size.cmp(&other.size))
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn solve_part_1(data: &str) -> String {
    let trees = TreeMatrix::new(data);
    let visible_trees = trees.get_visible_trees();
    let visible_count = 2 * trees.rows + 2 * trees.columns - 4 + visible_trees.len();

    visible_count.to_string()
}

fn solve_part_2(data: &str) -> String {
    let trees = TreeMatrix::new(data);
    trees.get_best_scenic_score().to_string()
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
        let data = "30373
25512
65332
33549
35390";

        assert_eq!(solve_part_2(data), "8");
    }
}

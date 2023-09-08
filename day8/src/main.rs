use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
struct Tree {
    height: i8,
    visible: bool,
}

impl Tree {
    fn new(height: i8) -> Self {
        Self {
            height,
            visible: false,
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.visible {
            write!(f, "\x1b[1m\x1b[31m{}\x1b[0m", self.height)
        } else {
            write!(f, "\x1b[37m{}", self.height)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn build(input: &str) -> Self {
        let mut rows = vec![];
        for row in input.lines() {
            let mut row_trees = vec![];
            for tree_height in row.chars() {
                row_trees.push(Tree::new(tree_height.to_string().parse().unwrap()));
            }
            rows.push(row_trees);
        }
        Forest { trees: rows }
    }

    fn to_columns(&self) -> Self {
        let mut trees = self.trees.clone();
        let mut iterators = vec![];
        for row in &mut trees {
            iterators.push(row.iter_mut());
        }
        let mut columns = vec![];
        for _ in 0..iterators.len() {
            let mut column = vec![];
            for iterator in &mut iterators {
                column.push(iterator.next().unwrap().clone());
            }
            columns.push(column);
        }
        Self { trees: columns }
    }

    fn calc_visability(&mut self) {
        // rows from left
        for row in &mut self.trees {
            let mut current_tallest_height = -1;
            for tree in row.iter_mut() {
                if tree.height > current_tallest_height {
                    tree.visible = true;
                    current_tallest_height = tree.height;
                }
            }
        }
        // rows from right
        for row in &mut self.trees {
            let mut current_tallest_height = -1;
            for tree in row.iter_mut().rev() {
                if tree.height > current_tallest_height {
                    tree.visible = true;
                    current_tallest_height = tree.height;
                }
            }
        }
        // columns from top
        let mut as_columns = self.to_columns();
        for column in &mut as_columns.trees {
            let mut current_tallest_height = -1;
            for tree in column.iter_mut() {
                if tree.height > current_tallest_height {
                    tree.visible = true;
                    current_tallest_height = tree.height;
                }
            }
        }
        // rows from right
        for column in &mut as_columns.trees {
            let mut current_tallest_height = -1;
            for tree in column.iter_mut().rev() {
                if tree.height > current_tallest_height {
                    tree.visible = true;
                    current_tallest_height = tree.height;
                }
            }
        }
        self.trees = as_columns.to_columns().trees;
    }

    fn count_visible(&self) -> usize {
        self.trees
            .iter()
            .flatten()
            .filter(|tree| tree.visible)
            .count()
    }
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = "".to_string();
        for row in &self.trees {
            result += &row.iter().map(|tree| tree.to_string()).collect::<String>();
            result += "\n";
        }
        write!(f, "{result}")
    }
}

fn main() {
    let mut forest = Forest::build(include_str!("../input.txt"));
    forest.calc_visability();
    println!("{forest}");
    println!("Part 1: {}", forest.count_visible());
}

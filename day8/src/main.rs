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

    fn scenic_score_in_direction(&self, trees_in_direction: &[Tree]) -> usize {
        let mut visible_trees = vec![];
        for tree in trees_in_direction {
            visible_trees.push(tree);
            if tree.height >= self.height {
                break;
            }
        }
        visible_trees.len()
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

    fn get_tree_on_position(&self, row: usize, column: usize) -> Tree {
        self.trees
            .get(row)
            .expect("Invalid row")
            .get(column)
            .expect("Invalid column")
            .clone()
    }

    fn get_trees_on_right_of_tree(&self, row: usize, column: usize) -> Vec<Tree> {
        let mut result = vec![];
        for (i, tree) in self.trees.get(row).expect("Invalid row").iter().enumerate() {
            if i > column {
                result.push(tree.clone());
            }
        }
        result
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

fn find_max_scenic_score(forest: &Forest) -> usize {
    // get arrays
    let trees_rows = Forest {
        trees: forest.trees.clone(),
    };
    let trees_rows_reversed = Forest {
        trees: forest
            .trees
            .clone()
            .into_iter()
            .map(|row| row.into_iter().rev().collect::<Vec<Tree>>())
            .collect(),
    };
    let trees_columns = Forest {
        trees: forest.to_columns().trees,
    };
    let trees_columns_reversed = Forest {
        trees: trees_columns
            .trees
            .clone()
            .into_iter()
            .map(|column| column.into_iter().rev().collect::<Vec<_>>())
            .collect(),
    };

    // println!(
    //     "{}\n{}\n{}\n{}",
    //     trees_rows, trees_rows_reversed, trees_columns, trees_columns_reversed
    // );
    let forest_size = forest.trees.len();
    // loop over every tree
    let mut all_scenic_scores = vec![];
    for row in 0..forest_size {
        for column in 0..forest_size {
            let trees_on_right = trees_rows.get_trees_on_right_of_tree(row, column);
            let trees_on_left =
                trees_rows_reversed.get_trees_on_right_of_tree(row, forest_size - column - 1);
            let trees_below = trees_columns.get_trees_on_right_of_tree(column, row);
            let trees_above =
                trees_columns_reversed.get_trees_on_right_of_tree(column, forest_size - row - 1);
            // calc scenic score
            let mut scenic_score = 1;
            scenic_score *= forest
                .get_tree_on_position(row, column)
                .scenic_score_in_direction(&trees_on_right);
            scenic_score *= forest
                .get_tree_on_position(row, column)
                .scenic_score_in_direction(&trees_on_left);
            scenic_score *= forest
                .get_tree_on_position(row, column)
                .scenic_score_in_direction(&trees_above);
            scenic_score *= forest
                .get_tree_on_position(row, column)
                .scenic_score_in_direction(&trees_below);
            all_scenic_scores.push(scenic_score);
        }
    }
    *all_scenic_scores.iter().max().expect("No scenic scores")
}

fn main() {
    let mut forest = Forest::build(include_str!("../input.txt"));
    forest.calc_visability();
    println!("{forest}");
    println!("Part 1: {}", forest.count_visible());
    println!("Part 2: {}", find_max_scenic_score(&forest));
}

use std::collections::HashSet;

fn generate_range_set(assignment: &str) -> HashSet<u32> {
    let bounds: Vec<u32> = assignment
        .split('-')
        .map(|bound| bound.parse().unwrap())
        .collect();
    HashSet::from_iter(bounds[0]..=*bounds.last().unwrap())
}

fn part1(input: &[Vec<&str>]) {
    println!(
        "{}",
        input
            .iter()
            .filter(|pair| {
                generate_range_set(pair[0]).is_subset(&generate_range_set(pair[1]))
                    || generate_range_set(pair[1]).is_subset(&generate_range_set(pair[0]))
            })
            .count()
    );
}

fn part2(input: &[Vec<&str>]) {
    println!(
        "{}",
        input
            .iter()
            .filter(|pair| {
                generate_range_set(pair[0]).intersection(&generate_range_set(pair[1])).count() != 0
            })
            .count()
    );
}

fn main() {
    let input: Vec<Vec<&str>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split(',').collect())
        .collect();

    part1(&input);
    part2(&input);
}

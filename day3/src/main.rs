fn get_priority_of_item(item: char) -> u32 {
    match item as u32 {
        65..=90 => item as u32 - 38,
        97..=122 => item as u32 - 96,
        _ => unreachable!("Invalid input"),
    }
}

fn part1(input: &[Vec<char>]) {
    println!(
        "The sum of the priorities is: {}",
        input
            .iter()
            .map(|rucksack| {
                let compartments = rucksack.split_at(rucksack.len() / 2);
                *compartments
                    .0
                    .iter()
                    .find(|item| compartments.1.contains(item))
                    .unwrap()
            })
            .map(get_priority_of_item)
            .sum::<u32>()
    );
}

fn part2(input: &[Vec<char>]) {
    println!(
        "The sum of the priorities is: {}",
        input
            .chunks(3)
            .map(|chunk| *chunk[0]
                .iter()
                .find(|item| chunk[1].contains(item) && chunk[2].contains(item))
                .unwrap())
            .map(get_priority_of_item)
            .sum::<u32>()
    );
}

fn main() {
    let input: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    part1(&input);
    part2(&input);
}

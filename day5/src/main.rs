fn part1(starting_chunks: &mut [Vec<char>; 9], instructions: &[(u8, u8, u8)]) {
    instructions.iter().for_each(|(amount, from, to)| {
        for _ in 0..*amount {
            let container = starting_chunks[(from - 1) as usize].pop().unwrap();
            starting_chunks[(to - 1) as usize].push(container);
        }
    });
    println!(
        "{}",
        starting_chunks
            .iter()
            .map(|stack| {
                if !stack.is_empty() {
                    *stack.iter().last().unwrap()
                } else {
                    ' '
                }
            })
            .collect::<String>()
    );
}

fn part2(starting_chunks: &mut [Vec<char>; 9], instructions: &[(u8, u8, u8)]) {
    instructions.iter().for_each(|(amount, from, to)| {
        let containers: Vec<char> = starting_chunks[(from - 1) as usize]
            .windows(*amount as usize)
            .last()
            .unwrap()
            .to_vec();

        for container in containers {
            starting_chunks[(to - 1) as usize].push(container);
        }
        for _ in 0..*amount {
            starting_chunks[(from - 1) as usize].pop();
        }
    });

    println!(
        "{}",
        starting_chunks
            .iter()
            .map(|stack| {
                if !stack.is_empty() {
                    *stack.iter().last().unwrap()
                } else {
                    ' '
                }
            })
            .collect::<String>()
    );
}

fn main() {
    let input_chunks = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut starting_chunks = [(); 9].map(|_| Vec::new());
    for line in input_chunks
        .0
        .lines()
        .take(input_chunks.0.lines().count() - 1)
    {
        for (char_index, char) in line.chars().enumerate() {
            if char_index > 0 && (char_index - 1) % 4 == 0 && char != ' ' {
                starting_chunks[(char_index + 1) / 4].insert(0, char);
            }
        }
    }
    let instructions: Vec<(u8, u8, u8)> = input_chunks
        .1
        .lines()
        .map(|line| {
            let line_elements = line.split(' ').collect::<Vec<&str>>();
            (
                line_elements[1].parse().unwrap(),
                line_elements[3].parse().unwrap(),
                line_elements[5].parse().unwrap(),
            )
        })
        .collect();

    part1(&mut starting_chunks.clone(), &instructions);
    part2(&mut starting_chunks.clone(), &instructions);
}

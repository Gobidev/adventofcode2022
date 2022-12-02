#[derive(Debug)]
enum Sign {
    Rock,
    Paper,
    Scissor,
    X,
    Y,
    Z,
}

fn part1(opponent_sign: &Sign, own_sign: &Sign) -> u32 {
    match opponent_sign {
        Sign::Rock => match own_sign {
            Sign::X => 4,
            Sign::Y => 8,
            Sign::Z => 3,
            _ => panic!("Invalid input"),
        }
        Sign::Paper => match own_sign {
            Sign::X => 1,
            Sign::Y => 5,
            Sign::Z => 9,
            _ => panic!("Invalid input"),
        }
        Sign::Scissor => match own_sign {
            Sign::X => 7,
            Sign::Y => 2,
            Sign::Z => 6,
            _ => panic!("Invalid input"),
        }
        _ => panic!("Invalid input"),
    }
}

fn part2(opponent_sign: &Sign, own_sign: &Sign) -> u32 {
    match opponent_sign {
        Sign::Rock => match own_sign {
            Sign::X => 3,
            Sign::Y => 4,
            Sign::Z => 8,
            _ => panic!("Invalid input"),
        }
        Sign::Paper => match own_sign {
            Sign::X => 1,
            Sign::Y => 5,
            Sign::Z => 9,
            _ => panic!("Invalid input"),
        }
        Sign::Scissor => match own_sign {
            Sign::X => 2,
            Sign::Y => 6,
            Sign::Z => 7,
            _ => panic!("Invalid input"),
        }
        _ => panic!("Invalid input"),
    }
}

fn main() {
    let input: Vec<Vec<Sign>> = include_str!("../input.txt").lines().map(|line| {
        line.split(' ').map(|val| match val {
            "A" => Sign::Rock,
            "X" => Sign::X,
            "B" => Sign::Paper,
            "Y" => Sign::Y,
            "C" => Sign::Scissor,
            "Z" => Sign::Z,
            _ => panic!("Invalid input")
        }).collect()
    }
    ).collect();

    let mut total_points: u32 = 0;
    for game in &input {
        total_points += part1(&game[0], &game[1]);
    }
    println!("Total points for part 1: {total_points}");

    let mut total_points: u32 = 0;
    for game in input {
        total_points += part2(&game[0], &game[1]);
    }
    println!("Total points for part 2: {total_points}");
}

use std::collections::HashSet;

fn find_marker(input: &str, windows_size: u8) -> usize {
    let mut last: Vec<char> = Vec::new();
    for (char_index, char) in input.chars().into_iter().enumerate() {
        let unique: HashSet<char> = HashSet::from_iter(last.iter().cloned());
        if unique.len() == windows_size as usize {
            return char_index;
        }
        last.push(char);
        if last.len() > windows_size as usize {
            last.remove(0);
        }
    }
    unreachable!("No marker found");
}

fn main() {
    let input = include_str!("../input.txt").lines().next().unwrap();
    println!("{}", find_marker(input, 4));
    println!("{}", find_marker(input, 14));
}

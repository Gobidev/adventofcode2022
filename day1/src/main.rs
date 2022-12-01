fn main() {
    let mut elf_calories: Vec<u32> = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect();
    println!(
        "Elf with the most calories has: {}",
        elf_calories.iter().max().unwrap()
    );
    elf_calories.sort_unstable();
    println!(
        "The three elves with the most calories have a combined amount of: {}",
        elf_calories.iter().rev().take(3).sum::<u32>()
    );
}

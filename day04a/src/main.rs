use std::collections::HashSet;

fn main() {
    let total = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let line = line.split_once(": ").unwrap().1;
            let (winning_numbers, held_numbers) = line.split_once(" | ").unwrap();

            let winning_numbers = winning_numbers
                .split(' ')
                .flat_map(|line| line.parse::<u8>())
                .collect::<HashSet<_>>();

            let wins = held_numbers
                .split(' ')
                .flat_map(|line| line.parse())
                .filter(|number| winning_numbers.contains(number))
                .count();

            match wins {
                0 => 0,
                _ => 2u32.pow(wins as u32 - 1),
            }
        })
        .sum::<u32>();

    println!("{total}");
}

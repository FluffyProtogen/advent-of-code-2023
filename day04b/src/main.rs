use std::collections::HashSet;

fn main() {
    let card_wins = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let line = line.split_once(": ").unwrap().1;
            let (winning_numbers, held_numbers) = line.split_once(" | ").unwrap();

            let winning_numbers = winning_numbers
                .split(" ")
                .flat_map(|line| line.parse::<u8>())
                .collect::<HashSet<_>>();

            held_numbers
                .split(" ")
                .flat_map(|line| line.parse())
                .filter(|number| winning_numbers.contains(number))
                .count() as u8
        })
        .collect::<Vec<_>>();

    let total = total_cards(&card_wins, 0, card_wins.len()) + card_wins.len() as u32;

    println!("{total}");
}

fn total_cards(card_wins: &[u8], index: usize, amount: usize) -> u32 {
    card_wins[index..index + amount]
        .iter()
        .enumerate()
        .map(|(i, wins)| total_cards(card_wins, index + i + 1, *wins as usize) + *wins as u32)
        .sum::<u32>()
}

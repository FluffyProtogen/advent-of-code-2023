use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let total = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let (cards, bet) = line.split_once(' ').unwrap();
            let hand = Hand::new(cards);
            let bet = bet.parse::<usize>().unwrap();
            (hand, bet)
        })
        .sorted_by(|first, second| first.0.cmp(&second.0))
        .enumerate()
        .map(|(index, (_, bet))| (index + 1) * bet)
        .sum::<usize>();
    println!("{total}");
}

#[derive(PartialEq, Eq, Debug)]
struct Hand<'a> {
    cards: &'a str,
    ty: Type,
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str) -> Self {
        Self {
            cards,
            ty: cards.into(),
        }
    }

    fn compare_cards(&self, other: &'a str) -> Ordering {
        self.cards
            .chars()
            .zip(other.chars())
            .find_map(
                |(this, other)| match card_value(this).cmp(&card_value(other)) {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Equal => None,
                    Ordering::Greater => Some(Ordering::Greater),
                },
            )
            .unwrap_or(Ordering::Equal)
    }
}

fn card_value(card: char) -> u8 {
    match card {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        'J' => 0,
        _ => unreachable!(),
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.ty.cmp(&other.ty) {
            Ordering::Equal => self.compare_cards(other.cards),
            ord => ord,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        let mut occurences = value.chars().fold(HashMap::new(), |mut map, c| {
            map.entry(c).and_modify(|e| *e += 1).or_insert(1);
            map
        });
        if occurences.len() != 1 {
            let jokers = *occurences.get(&'J').unwrap_or(&0);
            occurences.remove(&'J');
            let highest = occurences
                .iter_mut()
                .sorted_by_key(|(_, count)| **count)
                .last()
                .unwrap()
                .1;
            *highest += jokers;
        }
        match occurences.len() {
            1 => Self::Five,
            2 => match occurences.values().next().unwrap() {
                2 | 3 => Self::Full,
                _ => Self::Four,
            },
            3 => {
                let (first, second, third) = occurences.values().next_tuple().unwrap();
                if *first == 3 || *second == 3 || *third == 3 {
                    Self::Three
                } else {
                    Self::Two
                }
            }
            5 => Self::High,
            _ => Self::One,
        }
    }
}

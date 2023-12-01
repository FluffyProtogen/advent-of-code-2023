use substring::Substring;

const NUMBERS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let total = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut numbers = (0..line.len()).filter_map(|i| {
                let substring = line.substring(i, (i + 5).clamp(0, line.len()));
                if let Some(num) = substring.chars().nth(0).and_then(|char| char.to_digit(10)) {
                    return Some(num);
                }
                NUMBERS
                    .iter()
                    .find(|(number, _)| substring.starts_with(number))
                    .map(|(_, value)| *value)
            });
            let first = numbers.nth(0).unwrap();
            first * 10 + numbers.last().unwrap_or(first)
        })
        .sum::<u32>();
    println!("{total}");
}

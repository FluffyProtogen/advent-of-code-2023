fn main() {
    let total = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut numbers = line.chars().map(|char| char.to_digit(10)).flatten();
            let first = numbers.nth(0).unwrap();
            first * 10 + numbers.last().unwrap_or(first)
        })
        .sum::<u32>();
    println!("{total}");
}

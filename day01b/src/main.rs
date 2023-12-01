use substring::Substring;

fn main() {
    let total = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let numbers = get_numbers(line);
            let mut numbers = numbers.iter();
            let first = numbers.nth(0).unwrap();
            first * 10 + numbers.last().unwrap_or(first)
        })
        .sum::<u32>();
    println!("{total}");
}

fn get_numbers(line: &str) -> Vec<u32> {
    let mut chars = line.chars().collect::<Vec<_>>();
    chars.extend("____".chars()); // lazy to look for windows function that doesn't return exact size
    chars
        .windows(5) // three is longest at 5 chars
        .filter_map(|chunk| {
            if let Some(digit) = chunk[0].to_digit(10) {
                return Some(digit);
            } else {
                let text = chunk.iter().collect::<String>();
                if text.len() > 4 {
                    match text.substring(0, 5) {
                        "three" => return Some(3),
                        "seven" => return Some(7),
                        "eight" => return Some(8),
                        _ => {}
                    }
                }
                if text.len() > 3 {
                    match text.substring(0, 4) {
                        "four" => return Some(4),
                        "five" => return Some(5),
                        "nine" => return Some(9),
                        _ => {}
                    }
                }
                if text.len() > 2 {
                    match text.substring(0, 3) {
                        "one" => return Some(1),
                        "two" => return Some(2),
                        "six" => return Some(6),
                        _ => {}
                    }
                }
                None
            }
        })
        .collect()
}

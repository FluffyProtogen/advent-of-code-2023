const CHECK_LOCATIONS: [(i16, i16); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn main() {
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();

    let total = lines
        .iter()
        .map(|line| {
            let (_, numbers) = line.split(|l: char| !l.is_digit(10)).enumerate().fold(
                (0, vec![]),
                |(mut length_offset, mut vec), (index, number)| {
                    if let Ok(num) = number.parse::<u16>() {
                        vec.push((num, index + length_offset, number.len()));
                        length_offset += number.len();
                    }

                    (length_offset, vec)
                },
            );
            numbers
        })
        .enumerate()
        .flat_map(|(line_index, mut numbers)| {
            numbers.retain(|(_, index, length)| {
                (0..*length).any(|index_offset| {
                    CHECK_LOCATIONS.iter().any(|(x, y)| {
                        let Ok(x) = usize::try_from(*index as i16 + index_offset as i16 + x) else {
                            return false;
                        };
                        let Ok(y) = usize::try_from(line_index as i16 + y) else {
                            return false;
                        };

                        if !(0..lines.len()).contains(&y) || !(0..lines[0].len()).contains(&x) {
                            false
                        } else {
                            match lines[y].chars().nth(x) {
                                Some('@' | '*' | '%' | '#' | '/' | '&' | '$' | '+' | '-' | '=') => {
                                    true
                                }
                                _ => false,
                            }
                        }
                    })
                })
            });
            numbers
        })
        .fold(0, |acc, (number, ..)| acc + number as u32);

    println!("{total}");
}

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
        .enumerate()
        .flat_map(|(line_y, line)| {
            let mut number_offset = 0;
            let lines = &lines;
            line.split(|l: char| !l.is_digit(10))
                .filter_map(move |split| {
                    let Ok(number) = split.parse::<u16>() else {
                        number_offset += 1;
                        return None;
                    };
                    (0..split.len())
                        .any(|index_offset| {
                            CHECK_LOCATIONS.iter().any(|(x, y)| {
                                let (Ok(x), Ok(y)) = (
                                    usize::try_from(number_offset + x + index_offset as i16),
                                    usize::try_from(line_y as i16 + y),
                                ) else {
                                    return false;
                                };

                                if !(0..lines.len()).contains(&y)
                                    || !(0..lines[0].len()).contains(&x)
                                {
                                    false
                                } else {
                                    match lines[y].chars().nth(x) {
                                        Some(
                                            '@' | '*' | '%' | '#' | '/' | '&' | '$' | '+' | '-'
                                            | '=',
                                        ) => true,
                                        _ => false,
                                    }
                                }
                            })
                        })
                        .then_some({
                            number_offset += split.len() as i16 + 1;
                            number as u32
                        })
                })
        })
        .sum::<u32>();

    println!("{total}");
}

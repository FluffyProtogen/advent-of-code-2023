use itertools::Itertools;

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
            line.chars()
                .enumerate()
                .filter(|(_, chr)| *chr == '*')
                .map(move |(line_x, _)| (line_x, line_y))
        })
        .map(|(line_x, line_y)| {
            let total = CHECK_LOCATIONS
                .iter()
                .filter_map(|(x, y)| {
                    let x = usize::try_from(line_x as i16 + x).ok()?;
                    let y = usize::try_from(line_y as i16 + y).ok()?;
                    if !(0..lines.len()).contains(&y) || !(0..lines[0].len()).contains(&x) {
                        return None;
                    }
                    let result = find_number(&lines[y], x).map(|result| (result.0, result.1, y));
                    result
                })
                .unique_by(|key| (key.1, key.2))
                .collect_vec();
            if total.len() == 2 {
                total[0].0 as u64 * total[1].0 as u64
            } else {
                0
            }
        })
        .sum::<u64>();

    println!("{total}");
}

fn find_number(line: &str, position: usize) -> Option<(u16, u16)> {
    let chars = line.chars().collect_vec();
    let mut index = position;
    if !chars[position].is_numeric() {
        return None;
    }
    while chars[index].is_numeric() && index > 0 {
        index -= 1;
    }
    if !(index == 0 && chars[0].is_numeric()) {
        index += 1;
    }
    line.chars()
        .skip(index)
        .take_while(|c| c.to_digit(10).is_some())
        .collect::<String>()
        .parse()
        .ok()
        .map(|num| (num, index as u16))
}

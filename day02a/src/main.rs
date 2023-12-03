const RED_MAX: u16 = 12;
const GREEN_MAX: u16 = 13;
const BLUE_MAX: u16 = 14;

fn main() {
    let total = include_str!("../input.txt")
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let line = line.split(": ").nth(1).unwrap();
            line.split("; ")
                .flat_map(|color| color.split(", ").map(parse_colors))
                .all(|(value, color)| match color {
                    "red" => value <= RED_MAX,
                    "green" => value <= GREEN_MAX,
                    "blue" => value <= BLUE_MAX,
                    _ => unreachable!(),
                })
                .then_some(index as u16 + 1)
        })
        .sum::<u16>();

    println!("{total}");
}

fn parse_colors(color_set: &str) -> (u16, &str) {
    let (value, color) = color_set.split_once(' ').unwrap();
    (value.parse().unwrap(), color)
}

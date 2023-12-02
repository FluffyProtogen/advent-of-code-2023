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
                .map(|color| {
                    color.split(", ").map(|item| {
                        let (number, color) = item.split_once(' ').unwrap();
                        (number.parse::<u16>().unwrap(), color)
                    })
                })
                .find_map(|mut lines| {
                    lines.find(|(value, color)| match *color {
                        "red" => *value > RED_MAX,
                        "green" => *value > GREEN_MAX,
                        "blue" => *value > BLUE_MAX,
                        _ => unreachable!(),
                    })
                })
                .is_none()
                .then_some(index as u16 + 1)
        })
        .sum::<u16>();

    println!("{total}");
}

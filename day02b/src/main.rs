use std::collections::HashMap;

fn main() {
    let total = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let line = line.split(": ").nth(1).unwrap();
            line.split("; ")
                .flat_map(|color_set| color_set.split(", ").map(parse_colors))
                .fold(HashMap::new(), |mut map, (value, color)| {
                    map.entry(color)
                        .and_modify(|old_value| *old_value = std::cmp::max(*old_value, value))
                        .or_insert(value);
                    map
                })
                .values()
                .product::<u32>()
        })
        .sum::<u32>();

    println!("{total}");
}

fn parse_colors(color_set: &str) -> (u32, &str) {
    let (value, color) = color_set.split_once(' ').unwrap();
    (value.parse().unwrap(), color)
}

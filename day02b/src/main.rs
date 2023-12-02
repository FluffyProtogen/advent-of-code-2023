use std::collections::HashMap;

fn main() {
    let total = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let line = line.split(": ").nth(1).unwrap();
            line.split("; ")
                .flat_map(|color_set| {
                    color_set.split(", ").map(|item| {
                        let (value, color) = item.split_once(' ').unwrap();
                        (value.parse::<u32>().unwrap(), color)
                    })
                })
                .fold(HashMap::new(), |mut map, (value, color)| {
                    map.entry(color)
                        .and_modify(|old_value| *old_value = std::cmp::max(*old_value, value))
                        .or_insert(value);
                    map
                })
                .iter()
                .fold(1, |total, (_, value)| total * value)
        })
        .sum::<u32>();

    println!("{total}");
}

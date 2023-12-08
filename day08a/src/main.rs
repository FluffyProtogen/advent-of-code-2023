use std::collections::HashMap;

fn main() {
    let (directions, map) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let directions = directions
        .chars()
        .map(|c| match c {
            'R' => 1,
            'L' => 0,
            _ => unreachable!(),
        })
        .cycle();
    let map = map.lines().map(parse_key).collect::<HashMap<_, _>>();
    let mut current = "AAA";
    let total = directions.enumerate().find_map(|(count, direction)| {
        current = map[current][direction];
        (current == "ZZZ").then_some(count + 1)
    });
    println!("{total:?}");
}

fn parse_key(line: &str) -> (&str, [&str; 2]) {
    let (key, values) = line.split_once(" = ").unwrap();
    let (first, second) = values.split_once(", ").unwrap();
    let first = &first[1..];
    let second = &second[..second.len() - 1];
    (key, [first, second])
}

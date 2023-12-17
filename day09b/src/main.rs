fn main() {
    let total = include_str!("../input.txt")
        .lines()
        .map(extrapolate)
        .sum::<i64>();
    println!("{total}");
}

fn extrapolate(sequence: &str) -> i64 {
    let sequences = sequence
        .split(' ')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let mut sequences = vec![sequences];

    while !sequences.last().unwrap().iter().all(|c| *c == 0) {
        let sequence = sequences
            .last()
            .unwrap()
            .windows(2)
            .map(|s| s[1] - s[0])
            .collect();
        sequences.push(sequence);
    }

    sequences
        .iter()
        .rev()
        .scan(0, |mut state, sequence| {
            *state = *sequence.first().unwrap() - *state;
            Some(*state)
        })
        .last()
        .unwrap()
}

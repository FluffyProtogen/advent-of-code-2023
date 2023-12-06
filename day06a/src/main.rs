fn main() {
    let (times, distances) = include_str!("../input.txt").split_once('\n').unwrap();
    let times = times.split(' ').filter_map(|s| s.trim().parse().ok());
    let distances = distances.split(' ').filter_map(|s| s.trim().parse().ok());
    let possibilities = times
        .zip(distances)
        .map(|(time, distance)| get_possible_wins(time, distance))
        .product::<u32>();
    println!("{possibilities}");
}

fn get_possible_wins(total_time: u32, distance: u32) -> u32 {
    (0..total_time)
        .filter(|time| (total_time - time) * time > distance)
        .count() as u32
}

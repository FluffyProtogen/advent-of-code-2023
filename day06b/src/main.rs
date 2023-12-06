fn main() {
    let (total_time, distance) = include_str!("../input.txt").split_once('\n').unwrap();
    let total_time = total_time.split_once(": ").unwrap().1;
    let distance = distance.split_once(": ").unwrap().1;
    let total_time = total_time.replace(' ', "").parse().unwrap();
    let distance = distance.replace(' ', "").parse::<u64>().unwrap();
    let possibilities = (0..total_time)
        .filter(|time| (total_time - time) * time > distance)
        .count();
    println!("{possibilities}");
}

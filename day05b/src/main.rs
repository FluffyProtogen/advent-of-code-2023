use itertools::Itertools;

fn main() {
    let (seeds, lines) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let (_, seeds) = seeds.split_once(": ").unwrap();
    let seeds = seeds
        .split('\n')
        .flat_map(|split| split.split(' '))
        .filter_map(|num| num.parse().ok())
        .tuples()
        .flat_map(|(first, second)| first..=first + second);

    let maps = lines
        .split("\n\n")
        .map(|chunk| chunk.split_once(":\n").unwrap().1)
        .map(chunk_to_ranges)
        .collect();
    let almanac = Almanac { maps };
    let min = seeds.map(|seed| almanac.look_up(seed)).min().unwrap();
    println!("{min}");
}

struct Almanac {
    maps: Vec<Vec<MapRange>>,
}

impl Almanac {
    fn look_up(&self, number: u64) -> u64 {
        self.maps.iter().fold(number, |current, map| {
            map.iter()
                .find_map(|map| map.look_up(current))
                .unwrap_or(current)
        })
    }
}

struct MapRange {
    destination_start: u64,
    source_start: u64,
    count: u64,
}

impl MapRange {
    fn look_up(&self, number: u64) -> Option<u64> {
        (self.source_start..self.source_start + self.count)
            .contains(&number)
            .then(|| (number - self.source_start) + self.destination_start)
    }
}

fn chunk_to_ranges(chunk: &str) -> Vec<MapRange> {
    chunk
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let destination_start = split.next().unwrap().parse().unwrap();
            let source_start = split.next().unwrap().parse().unwrap();
            let count = split.next().unwrap().parse().unwrap();
            MapRange {
                destination_start,
                source_start,
                count,
            }
        })
        .collect()
}

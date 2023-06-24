use std::ops::RangeInclusive;

use itertools::Itertools;

fn parse_range(s: &str) -> RangeInclusive<u64> {
    let (start, end) = s.split_once('-').unwrap();
    let (start, end) = (start.parse().unwrap(), end.parse().unwrap());

    start..=end
}

fn do_range_completely_overlap(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.start() >= b.start() && a.end() <= b.end() || a.start() <= b.start() && a.end() >= b.end()
}

fn main() {
    let input = include_str!("../input.txt");

    let result = input
        .lines()
        .map(|l| l.split(',').map(parse_range).collect_vec())
        .inspect(|r| assert_eq!(r.len(), 2))
        .map(|r| {
            let mut r = r.into_iter();
            (r.next().unwrap(), r.next().unwrap())
        })
        .filter(|(a, b)| do_range_completely_overlap(a, b))
        .count();

    println!("{}", result);
}

use std::collections::HashSet;
use itertools::Itertools;

fn get_itemtype_priority(c: char) -> u64 {
    match c {
        'a'..='z' => (c as u8 - b'a') as u64 + 1,
        'A'..='Z' => (c as u8 - b'A') as u64 + 27,
        _ => panic!("Invalid char: {}", c)
    }
}

fn get_rucksacks_common_itemtype_priority<'a, I>(line: I) -> u64 
where
    I: Iterator<Item = &'a str>
{
    let rucksacks = line
        .map(|str| str.chars())
        .map(HashSet::<char>::from_iter)
        .collect_vec();

    assert!(rucksacks.len() == 3);

    let common_itemtypes = rucksacks
        .into_iter()
        .reduce(|acc, rucksack| {
            acc.intersection(&rucksack).cloned().collect()
        })
        .unwrap();

    assert!(common_itemtypes.len() == 1, "common_itemtypes: {:?}", common_itemtypes);

    common_itemtypes
        .into_iter()
        .map(get_itemtype_priority)
        .sum()
}


fn main() {
    let input = include_str!("../input.txt");

    let result = &input
        .lines()
        .chunks(3)
        .into_iter()
        .map(get_rucksacks_common_itemtype_priority)
        .sum::<u64>();

    println!("{}", result)
}

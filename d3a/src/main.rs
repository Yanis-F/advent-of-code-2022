use std::collections::HashSet;

fn get_char_priority(c: &char) -> u64 {
    match c {
        'a'..='z' => (*c as u8 - b'a') as u64 + 1,
        'A'..='Z' => (*c as u8 - b'A') as u64 + 27,
        _ => panic!("Invalid char: {}", c)
    }
}

fn get_rucksack_duplicate_priority(line: &str) -> u64 {
    let pocket_size = line.len() / 2;
    let pocket_a = &line[..pocket_size];
    let pocket_b = &line[pocket_size..];

    assert_eq!(pocket_a.len(), pocket_b.len());

    let pocket_a = HashSet::<char>::from_iter(pocket_a.chars());
    let pocket_b = HashSet::<char>::from_iter(pocket_b.chars());

    let duplicates = pocket_a.intersection(&pocket_b);

    assert!(duplicates.clone().count() == 1);

    duplicates.into_iter()
        .map(get_char_priority)
        .sum()
}


fn main() {
    let input = include_str!("../input.txt");

    let result = input
        .lines()
        .map(get_rucksack_duplicate_priority)
        .sum::<u64>();

    println!("{}", result);
}

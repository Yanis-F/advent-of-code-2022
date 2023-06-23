use std::{path::Path, io::BufRead};


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];
    let filepath = Path::new(&filename);

    if !filepath.exists() {
        println!("{} does not exist", filename);
        return;
    }
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);
    let lines = reader.lines();

    let mut top3 = vec![0, 0, 0];

    let mut current = 0;
    for line in lines.map(|l| l.unwrap()) {
        if line.is_empty() {
            top3.push(current);
            top3.sort_unstable_by(|a, b| b.cmp(a));
            top3.pop();

            current = 0;
        } else {
            let value = line.parse::<u64>().unwrap();
            current += value;
        }
    }

    println!("{}", top3.iter().sum::<u64>());
}

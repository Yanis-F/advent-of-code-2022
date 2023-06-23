use std::{io::BufRead, path::Path};


fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }


    let filepath = Path::new(&args[1]);
    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    let mut max = 0;
    let mut current_numbers = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            let sum = current_numbers.iter().sum::<u64>();
            if sum > max {
                max = sum;
            }
            current_numbers.clear();
            continue;
        } else {
            let num = line.parse::<u64>().unwrap();
            current_numbers.push(num);
        }
    }

    println!("{}", max);
    Ok(())
}

use std::{str::FromStr, convert::Infallible, fmt::{Display, Arguments}};
use itertools::{Itertools, enumerate};
use regex::Regex;

type Crate = char;

#[derive(Debug)]
struct Cargo {
    pub stacks: Vec<Stack>,
}

impl FromStr for Cargo {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = Vec::new();

        for line in s.lines() {
            for (stack_id, cargo_str) in enumerate(&line.chars().chunks(4)) {
                if stacks.len() < stack_id + 1 {
                    stacks.push(Stack { crates: Vec::new() });
                }

                let cargo = cargo_str.into_iter().nth(1).unwrap();

                if cargo == '1' {
                    break;
                }
                if cargo != ' ' {
                    stacks[stack_id].crates.push(cargo);
                }
            }
        }

        for s in stacks.iter_mut() {
            s.crates.reverse();
        }
        Ok(Self { stacks })
    }
}

#[derive(Debug)]
struct Stack {
    pub crates: Vec<Crate>,
}


#[derive(Debug)]
struct Move {
    pub count: u64,
    pub from: usize,
    pub to: usize,
}

impl FromStr for Move {
    type Err = Infallible;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        let captures = re.captures(str).unwrap();

        let count = captures.get(1).unwrap().as_str().parse().unwrap();
        let from  = captures.get(2).unwrap().as_str().parse().unwrap();
        let to    = captures.get(3).unwrap().as_str().parse().unwrap();
        Ok(Self { count, from, to })
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {} from {} to {}", self.count, self.from, self.to)
    }
}


fn main() {
    let input = include_str!("../input.txt");

    let (cargo_state, moves) = input.split_once("\n\n").unwrap();

    let mut cargo = Cargo::from_str(cargo_state).unwrap();

    let moves = moves.lines().map(|l| Move::from_str(l).unwrap());
        
    for m in moves {
        // println!("{}", m);
        for _ in 0..m.count {
            let c = cargo.stacks[m.from - 1].crates.pop().unwrap();
            cargo.stacks[m.to - 1].crates.push(c);
        }
    }

    let output = cargo.stacks.into_iter()
        .map(|s| *s.crates.last().unwrap())
        .join("");

    println!("{}", output)
}


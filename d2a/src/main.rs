#[derive(Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn from_enemy_char(c: char) -> Move {
        match c {
            'a' | 'A' => Move::Rock,
            'b' | 'B' => Move::Paper,
            'c' | 'C' => Move::Scissors,
            _ => panic!("Invalid enemy move: '{}'", c),
        }
    }

    pub fn from_ally_char(c: char) -> Move {
        match c {
            'x' | 'X' => Move::Rock,
            'y' | 'Y' => Move::Paper,
            'z' | 'Z' => Move::Scissors,
            _ => panic!("Invalid ally move: '{}'", c),
        }
    }

    pub fn get_base_point(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    pub fn get_round_point_against(&self, enemy_move: &Move) -> u32 {
        match get_result(self, enemy_move) {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        }

    }
}

enum GameResult {
    Win,
    Draw,
    Lose,
}

fn get_result(our_move: &Move, enemy_move: &Move) -> GameResult {
    if our_move == enemy_move {
        GameResult::Draw
    } else if *our_move == Move::Rock && *enemy_move == Move::Scissors
        || *our_move == Move::Paper && *enemy_move == Move::Rock
        || *our_move == Move::Scissors && *enemy_move == Move::Paper {
        GameResult::Win
    } else {
        GameResult::Lose
    }
}



fn main() {
    let input = include_str!("../input.txt");

    let score = input
        .lines()
        // .inspect(|line| println!("{}", line))
        .map(|line| (line.chars().next().unwrap(), line.chars().nth(2).unwrap()))
        .map(|(enemy, ally)| (Move::from_enemy_char(enemy), Move::from_ally_char(ally)))
        .map(|(ennemy, ally)| {
            (ally.get_base_point(), ally.get_round_point_against(&ennemy))
        })
        // .inspect(|(base, round)| println!("{} + {}", base, round))
        .map(|(base, round)| base + round)
        .sum::<u32>();

    println!("{}", score);
}

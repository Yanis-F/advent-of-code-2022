#[derive(Eq, PartialEq, Clone, Copy)]
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

    pub fn from_enemy_move_and_outcome(enemy_move: &Move, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Draw => *enemy_move,
            Outcome::Win => match enemy_move {
                Move::Rock     => Move::Paper,
                Move::Paper    => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Outcome::Lose => match enemy_move {
                Move::Rock     => Move::Scissors,
                Move::Paper    => Move::Rock,
                Move::Scissors => Move::Paper,
            },
        }
    }

    pub fn get_base_point(&self) -> u32 {
        match self {
            Move::Rock     => 1,
            Move::Paper    => 2,
            Move::Scissors => 3,
        }
    }

    pub fn get_round_point_against(&self, enemy_move: &Move) -> u32 {
        match get_result(self, enemy_move) {
            Outcome::Win  => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }

    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn from_char(c: char) -> Self {
        match c {
            'x' | 'X' => Outcome::Lose,
            'y' | 'Y' => Outcome::Draw,
            'z' | 'Z' => Outcome::Win,
            _ => panic!("Invalid outcome: '{}'", c),
        }
    }
}

fn get_result(our_move: &Move, enemy_move: &Move) -> Outcome {
    if our_move == enemy_move {
        Outcome::Draw
    } else if *our_move == Move::Rock && *enemy_move == Move::Scissors
        || *our_move == Move::Paper && *enemy_move == Move::Rock
        || *our_move == Move::Scissors && *enemy_move == Move::Paper {
        Outcome::Win
    } else {
        Outcome::Lose
    }
}



fn main() {
    let input = include_str!("../input.txt");

    let score = input
        .lines()
        // .inspect(|line| println!("{}", line))
        .map(|line| (line.chars().next().unwrap(), line.chars().nth(2).unwrap()))
        .map(|(enemy, outcome)| {
            let enemy_move = Move::from_enemy_char(enemy);
            let outcome = Outcome::from_char(outcome);

            (
                enemy_move,
                Move::from_enemy_move_and_outcome(&enemy_move, outcome)
            )
        })
        .map(|(ennemy, ally)| {
            (ally.get_base_point(), ally.get_round_point_against(&ennemy))
        })
        // .inspect(|(base, round)| println!("{} + {}", base, round))
        .map(|(base, round)| base + round)
        .sum::<u32>();

    println!("{}", score);
}

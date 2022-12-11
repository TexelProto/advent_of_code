use std::str::FromStr;

trait Score {
    fn score(&self) -> usize;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn vs(self, other: Move) -> Outcome {
        use Move::*;

        if self == other {
            Outcome::Draw
        } else if self == Rock && other == Scissors
            || self == Paper && other == Rock
            || self == Scissors && other == Paper
        {
            Outcome::Win
        } else {
            Outcome::Loose
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err(s.to_owned()),
        }
    }
}

impl Score for Move {
    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Loose,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(s.to_owned()),
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loose => 0,
        }
    }
}

fn get_move(opponent: Move, outcome: Outcome) -> Move {
    [Move::Rock, Move::Paper, Move::Scissors]
        .into_iter()
        .filter(move |mov| mov.vs(opponent) == outcome)
        .next()
        .unwrap()
}

pub fn task1(input: String) {
    let score = input
            .lines()
            .map(|s| s.trim().split_at(1))
            .map(|(a, b)| {
                (
                    Move::from_str(a.trim()).unwrap(),
                    Move::from_str(b.trim()).unwrap(),
                )
            })
            .map(|t| t.0.score() + t.0.vs(t.1).score())
            .sum::<usize>();

        println!("Score: {}", score);
    }
    pub fn task2(input: String) {
        let score = input
            .lines()
            .map(|s| s.trim().split_at(1))
            .map(|(a, b)| {
                (
                    Move::from_str(a.trim()).unwrap(),
                    Outcome::from_str(b.trim()).unwrap(),
                )
            })
            .map(|(opp_move, outcome)| get_move(opp_move, outcome).score() + outcome.score())
            .sum::<usize>();

        println!("Score: {}", score);
    }

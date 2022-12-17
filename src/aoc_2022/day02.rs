use std::str::FromStr;
use crate::input::Linewise;

trait Score {
    fn score(&self) -> u64;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Move {
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
    fn score(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub struct MoveMove(Move, Move);

impl FromStr for MoveMove {
    type Err = <Move as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_at(1);
        let a = Move::from_str(a)?;
        let b = Move::from_str(&b[1..])?;
        Ok(Self(a, b))
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
    fn score(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loose => 0,
        }
    }
}

pub struct MoveOutcome(Move, Outcome);

impl FromStr for MoveOutcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_at(1);
        let a = Move::from_str(a)?;
        let b = Outcome::from_str(&b[1..])?;
        Ok(Self(a, b))
    }
}

fn get_move(opponent: Move, outcome: Outcome) -> Move {
    [Move::Rock, Move::Paper, Move::Scissors]
        .into_iter()
        .filter(move |mov| mov.vs(opponent) == outcome)
        .next()
        .unwrap()
}

pub fn task1(input: Linewise<MoveMove>) -> Result<u64, String> {
    let mut score = 0_u64;
    crate::for_input!(input, |t| {
        score += t.0.score() + t.0.vs(t.1).score();
    });

    Ok(score)
}

pub fn task2(input: Linewise<MoveOutcome>) -> Result<u64, String> {
    let mut score = 0;
    crate::for_input!(input, |t| {
        score += get_move(t.0, t.1).score() + t.1.score();
    });

    Ok(score)
}

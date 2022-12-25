use std::{str::FromStr, iter::Iterator, num::ParseIntError};

use crate::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Invalid change format '{0}'")]
    InvalidChange(String),
}

#[derive(Debug)]
pub enum Change {
    AddX(isize),
    Noop,
}

impl Change {
    fn delay(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

impl FromStr for Change {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "noop" => Ok(Self::Noop),
            "addx" => {
                let num = s[5..].parse()?;
                Ok(Self::AddX(num))
            },
            s => Err(Error::InvalidChange(s.to_owned()))
        }
    }
}

fn get_samples(input: Linewise<Change>) -> Result<Vec<isize>, Error> {
    let mut acc = 1_isize;
    let mut samples = Vec::new();
    crate::for_input!(input, |c| {
        for _ in 0..c.delay() {
            samples.push(acc);
        }

        if let Change::AddX(x) = c {
            acc += x;
        }
    });
    Ok(samples)
}

pub fn task1(input: Linewise<Change>) -> Result<isize, Error> {
    let samples = get_samples(input)?;

    let result = samples.into_iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(u,x)| (u + 1) as isize * x)
        .sum::<isize>();

    Ok(result)
}

pub fn task2(input: Linewise<Change>) -> Result<String, Error> {
    let samples = get_samples(input)?;
    let mut samples = samples.into_iter();
    let mut buffer = String::with_capacity(252);

    for i in 0_isize .. 240 {
        let x = i % 40;
        if x == 0 {
            buffer.push_str("\r\n");
        }

        let sample = samples.next().unwrap();
        let c = if sample.abs_diff(x) <= 1 {
            '#'
        } else {
            ' '
        };
        buffer.push(c)
    }
    
    Ok(buffer)
}
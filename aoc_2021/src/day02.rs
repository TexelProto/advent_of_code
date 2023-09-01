use std::str::FromStr;

use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Invalid move input '{0}'")]
    ParseMove(String),
}

#[derive(Debug)]
pub enum Move {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_assert!(s.is_ascii());
        let bytes = s.as_bytes();
        let split = bytes.iter().position(|b| *b == b' ')
            .ok_or_else(|| Error::ParseMove(s.to_string()))?;
        let (prefix, value) = s.split_at(split);

        let value = u32::from_str(value.trim())?;

        let m = match prefix {
            "forward" => Self::Forward(value),
            "down" => Self::Down(value),
            "up" => Self::Up(value),
            _ => return Err(Error::ParseMove(value.to_string())),
        };

        Ok(m)
    }
}

pub fn task1(input: Linewise<Move>) -> Result<u32, Error> {
    let mut horizontal = 0;
    let mut depth = 0;

    for m in input {
        match m? {
            Move::Forward(n) => horizontal += n,
            Move::Down(n) => depth += n,
            Move::Up(n) => depth -= n,
        };
    }

    let score = horizontal * depth;
    Ok(score)
}

pub fn task2(input: Linewise<Move>) -> Result<u32, Error> {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for m in input {
        match m? {
            Move::Forward(n) => {
                horizontal += n;
                depth += n * aim;
            }
            Move::Down(n) => aim += n,
            Move::Up(n) => aim -= n,
        };
    }

    let score = horizontal * depth;
    Ok(score)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "forward 5
down 5
forward 8
up 3
down 8
forward 2
".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 150);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 900);
    }
}

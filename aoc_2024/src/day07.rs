use common::input::Linewise;
use common::iter_ext::TryIterator;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Missing separator ':' in input")]
    MissingSeparator,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub struct Equation {
    result: u64,
    values: Vec<u64>,
}

impl FromStr for Equation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prev, after) = s.split_once(':').ok_or(Error::MissingSeparator)?;

        let result = u64::from_str(prev)?;
        let values = after[1..].split(' ').map(u64::from_str).try_collect2()?;

        Ok(Self { result, values })
    }
}

const OPERATOR_ADD: u8 = 0;
const OPERATOR_MULTIPLY: u8 = 1;
const OPERATOR_CONCAT: u8 = 2;

fn next_permutation(operators: &mut [u8], kinds: u8) -> bool {
    for i in 0..operators.len() {
        operators[i] += 1;

        if operators[i] < kinds {
            return true;
        }

        operators[i] = 0;
    }
    false
}

fn evaluate_with_operators(values: &[u64], operators: &[u8]) -> u64 {
    let mut accumulator = values[0];

    for (index, &operator) in operators.iter().enumerate() {
        let value = values[index + 1];
        accumulator = match operator {
            OPERATOR_ADD => accumulator + value,
            OPERATOR_MULTIPLY => accumulator * value,
            OPERATOR_CONCAT => concat(accumulator, value),
            _ => unreachable!(),
        }
    }

    accumulator
}

fn concat(mut lhs: u64, rhs: u64) -> u64 {
    let mut remain = rhs;

    while remain > 0 {
        remain /= 10;
        lhs *= 10;
    }

    lhs + rhs
}

pub fn task1(input: Linewise<Equation>) -> Result<u64, Error> {
    let mut total = 0;
    let mut operator_buffer = [0u8; 32];
    for equation in input {
        let equation = equation?;

        let operator_count = equation.values.len() - 1;
        let mut operators = &mut operator_buffer[..operator_count];
        operators.fill(0);

        loop {
            let result = evaluate_with_operators(&equation.values, &operators);
            if result == equation.result {
                total += result;
                break;
            }

            if next_permutation(&mut operators, 2) == false {
                break;
            }
        }
    }
    Ok(total)
}

pub fn task2(input: Linewise<Equation>) -> Result<u64, Error> {
    let mut total = 0;
    let mut operator_buffer = [0u8; 32];
    for equation in input {
        let equation = equation?;

        let operator_count = equation.values.len() - 1;
        let mut operators = &mut operator_buffer[..operator_count];
        operators.fill(0);

        loop {
            let result = evaluate_with_operators(&equation.values, &operators);
            if result == equation.result {
                total += result;
                break;
            }

            if next_permutation(&mut operators, 3) == false {
                break;
            }
        }
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 3749);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 11387);
    }
}

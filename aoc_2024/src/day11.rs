use ahash::{HashMap, HashMapExt};
use common::iter_ext::TryIterator;
use std::mem::swap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

pub fn task1(input: String) -> Result<usize, Error> {
    let mut read = input
        .split(' ')
        .map(u64::from_str)
        .try_collect2::<Vec<_>>()?;
    let mut write = vec![];

    for _step in 0..25 {
        for &stone in &read {
            if stone == 0 {
                write.push(1);
            } else if let Some((l, r)) = split_even_digits(stone) {
                write.push(l);
                write.push(r);
            } else {
                write.push(stone * 2024);
            }
        }

        swap(&mut read, &mut write);
        write.clear();
    }

    Ok(read.len())
}

fn split_even_digits(stone: u64) -> Option<(u64, u64)> {
    let exponent = stone.ilog10() + 1;
    if exponent % 2 == 1 {
        return None;
    }

    let factor = 10u64.pow(exponent / 2);

    let left = stone / factor;
    let right = stone - (left * factor);

    Some((left, right))
}

pub fn task2(input: String) -> Result<u64, Error> {
    let mut read = HashMap::<u64, u64>::new();
    for item in input.split(' ').map(u64::from_str) {
        let value = item?;
        *read.entry(value).or_default() += 1;
    }

    let mut write = HashMap::new();

    for _step in 0..75 {
        for (&value, &count) in &read {
            if value == 0 {
                *write.entry(1).or_default() += count;
            } else if let Some((l, r)) = split_even_digits(value) {
                *write.entry(l).or_default() += count;
                *write.entry(r).or_default() += count;
            } else {
                let new_value = value * 2024;
                *write.entry(new_value).or_default() += count;
            }
        }

        swap(&mut read, &mut write);
        write.clear();
    }

    Ok(read.into_values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"125 17";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 55312);
    }
    // #[test]
    // fn test_task2() {
    //     let buf = std::io::BufReader::new(INPUT);
    //     let result = task2(Input::parse(buf).unwrap());
    //     let val = result.unwrap();
    //     assert_eq!(val, 0);
    // }
}

use std::collections::hash_map::Entry;
use common::input::Linewise;
use std::num::ParseIntError;
use std::str::FromStr;
use ahash::{HashMap, HashMapExt};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

#[derive(Debug, Copy, Clone)]
pub struct Pair(u32, u32);

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug_assert!(s.is_ascii());
        let left = s[..5].parse()?;
        let right = s[8..13].parse()?;
        Ok(Self(left, right))
    }
}

pub fn task1(mut input: Linewise<Pair>) -> Result<u32, Error> {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);

    input.try_for_each(|pair| {
        let pair = pair?;
        left.push(pair.0);
        right.push(pair.1);
        Ok::<(), Error>(())
    })?;

    left.sort();
    right.sort();

    let len = left.len();
    let total = (0..len).map(|i| left[i].abs_diff(right[i])).sum();

    Ok(total)
}

pub fn task2(mut input: Linewise<Pair>) -> Result<u32, Error> {
    let mut list = Vec::with_capacity(1000);
    let mut occurences = HashMap::with_capacity(1000);

    input.try_for_each(|pair| {
        let pair = pair?;
        list.push(pair.0);

        match occurences.entry(pair.1) {
            Entry::Occupied(mut o) => {
                *o.get_mut() += 1;
            },
            Entry::Vacant(mut v) => {
                v.insert(1);
            },
        }

        Ok::<(), Error>(())
    })?;

    let mut total = 0;
    for val in list {
        let Some(occ) = occurences.get(&val) else {
            continue;
        };

        total += val * occ;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 11);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 0);
    }
}

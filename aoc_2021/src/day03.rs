use std::{str::FromStr, num::ParseIntError};

use common::{input::Linewise, iter_ext::try_collect};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Number(u16);

impl std::ops::Deref for Number {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Number {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let u = u16::from_str_radix(s, 2)?;
        Ok(Self(u))
    }
}

pub fn task1(input: Linewise<Number>) -> Result<u32, ParseIntError> {
    task1_core::<12>(input)
}

fn task1_core<const WIDTH: usize>(input: Linewise<Number>) -> Result<u32, ParseIntError> {
    let mut bit_count = [0_i16; WIDTH];
    for i in input {
        let i = i?;
        for bit in 0..WIDTH {
            let mask = 1_u16 << bit;
            if *i & mask != 0 {
                bit_count[bit] += 1;
            } else {
                bit_count[bit] -= 1;
            }
        }
    }

    let mut gamma = 0;
    for bit in 0..WIDTH {
        if bit_count[bit] > 0 {
            gamma |= 1 << bit;
        }
    }

    let epsilon = !gamma & ((1 << WIDTH) - 1);
    let score = gamma * epsilon;
    Ok(score)
}

pub fn task2(input: Linewise<Number>) -> Result<u32, ParseIntError> {
    task2_core::<12>(input)
}

fn task2_core<const WIDTH: usize>(input: Linewise<Number>) -> Result<u32, ParseIntError> {
    let numbers: Vec<_> = try_collect(input)?;

    let mut o2_numbers = numbers.clone();
    filter_list::<WIDTH>(&mut o2_numbers, false);

    let mut co2_numbers = numbers;    
    filter_list::<WIDTH>(&mut co2_numbers, true);

    let score = *o2_numbers[0] as u32 * *co2_numbers[0] as u32;
    Ok(score)    
}

fn filter_list<const WIDTH: usize>(numbers: &mut Vec<Number>, invert: bool) {
    for bit in (0..WIDTH).rev() {
        let mask = 1 << bit;

        let bit_count = numbers.iter().cloned().fold(0_i16, |acc,n|{
            if *n & mask != 0 { 
                acc+1
            } else {
                acc-1
            }
        });

        let require_one = (bit_count >= 0) ^ invert;

        numbers.retain(move |num| {
            let has_one = **num & mask != 0;
            has_one == require_one
        });

        if numbers.len() <= 1 { break; }
    }
    assert_eq!(numbers.len(), 1);
}


#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1_core::<5>(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 198);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2_core::<5>(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 230);
    }
}

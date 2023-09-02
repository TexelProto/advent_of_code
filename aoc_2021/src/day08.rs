use std::str::FromStr;

use common::{iter_ext::try_collect, input::Linewise};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Input was missing a delimiter: '{0}'")]
    MissingDelimiter(&'static str),
    #[error("Input contained an unexpected character: '{0}'")]
    UnexpectedChar(char),
}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct Segments: u8 {
        const A = 1;
        const B = 2;
        const C = 4;
        const D = 8;
        const E = 16;
        const F = 32;
        const G = 64;
    }
}

impl FromStr for Segments {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n = Self::empty();
        for c in s.chars() {
            let flag = match c {
                'a' => Segments::A,
                'b' => Segments::B,
                'c' => Segments::C,
                'd' => Segments::D,
                'e' => Segments::E,
                'f' => Segments::F,
                'g' => Segments::G,
                _ => return Err(Error::UnexpectedChar(c)),
            };
            n |= flag;
        }
        Ok(n)
    }
}

impl Segments {
    fn active_bit_count(&self) -> u8 {
        self.0.0.count_ones() as u8
    }
}

#[derive(Debug)]
pub struct Input {
    examples: Vec<Segments>,
    active: Vec<Segments>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (examples, active) = s.split_once("|")
            .ok_or(Error::MissingDelimiter("|"))?;

        let examples = examples.split_ascii_whitespace()
            .map(Segments::from_str);
        let active = active.split_ascii_whitespace()
            .map(Segments::from_str);
        
        Ok(Self {
            examples: try_collect(examples)?,
            active: try_collect(active)?
        })
    }
}

pub fn task1(input: Linewise<Input>) -> Result<i32, Error> {
    const COOL_NUMBERS: [u8; 4] = [2,3,4,7];
    let mut cool_number_count = 0;
    for i in input {
        let i = i?;
        for segments in i.active {
            let num = segments.active_bit_count();
            if COOL_NUMBERS.contains(&num) {
                cool_number_count += 1;
            }
        }
    }
    Ok(cool_number_count)
}

pub fn task2(input: Linewise<Input>) -> Result<u32, Error> {

    fn take_with_bits(nums: &mut Vec<Segments>, active: u8) -> Result<Segments, Error> {       
        take(nums, move |n| n.active_bit_count() == active)
    }
    fn take(nums: &mut Vec<Segments>, f: impl Fn(&Segments) -> bool) -> Result<Segments, Error> {
        for i in 0..nums.len() {
            if !f(&nums[i]) { continue; }
            let n = nums.swap_remove(i);
            return Ok(n);
        }
        todo!()
    }

    let mut sum = 0;
    for i in input {
        let mut i = i?;
        
        let one = take_with_bits(&mut i.examples, 2)?;
        let four = take_with_bits(&mut i.examples, 4)?;
        let seven = take_with_bits(&mut i.examples, 3)?;
        let eight = take_with_bits(&mut i.examples, 7)?;

        let top_middle = seven & !one;
        let mostly_nine = four | top_middle;
        let nine = take(&mut i.examples, move |n| *n & mostly_nine == mostly_nine)?;

        // numbers with 6 active parts
        let zero = take(&mut i.examples, move |n| {
            n.active_bit_count() == 6 && (*n & one == one)
        })?;
        let six = take_with_bits(&mut i.examples, 6)?;

        // numbers with 5 active parts
        let three = take(&mut i.examples, move |n| {
            n.active_bit_count() == 5 && (*n & one == one)
        })?;
        let five = take(&mut i.examples, move |n| {
            n.active_bit_count() == 5 && (*n & six == *n)
        })?;
        let two = take_with_bits(&mut i.examples, 5)?;
        
        for pos in 0..4 {
            let seg = i.active[pos];

            let value = 
            if seg == zero { 0 }
            else if seg == one { 1 } 
            else if seg == two { 2 } 
            else if seg == three { 3 } 
            else if seg == four { 4 } 
            else if seg == five { 5 } 
            else if seg == six { 6 } 
            else if seg == seven { 7 } 
            else if seg == eight { 8 } 
            else if seg == nine { 9 } 
            else { unreachable!() };

            let pos_factor = 10_u32.pow(3 - pos as u32);
            sum += value * pos_factor;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 26);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 61229);
    }
}

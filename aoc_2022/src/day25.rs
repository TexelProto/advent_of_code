use std::{iter::Sum, fmt::{Display, Write}, str::FromStr, convert::Infallible};

use common::input::Linewise;

#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Digit {
    DoubleMinus = -2,
    Minus = -1,
    Zero = 0,
    One = 1,
    Two = 2,
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Digit::DoubleMinus => '=',
            Digit::Minus => '-',
            Digit::Zero => '0',
            Digit::One => '1',
            Digit::Two => '2',
        };
        f.write_char(c)
    }
}

static ALL_DIGIS: [Digit; 5] = [
    Digit::DoubleMinus,
    Digit::Minus,
    Digit::Zero,
    Digit::One,
    Digit::Two,
];

impl From<char> for Digit {
    fn from(value: char) -> Self {
        match value {
            '=' => Self::DoubleMinus,
            '-' => Self::Minus,
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            _ => panic!("Invalid char {value}"),
        }
    }
}

impl Digit {
    fn positional_value(self, pos: usize) -> i64 {
        5_i64.pow(pos as u32) * (self as i64)
    }
}

impl From<Digit> for i64 {
    fn from(value: Digit) -> Self {
        value as i64
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number(Vec<Digit>);

impl FromStr for Number {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Number(s.chars().map(Digit::from).collect()))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for digit in &self.0 {
            digit.fmt(f)?;
        }
        Ok(())
    }
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.map(i64::from).sum::<i64>().into()
    }
}

impl From<Number> for i64 {
    fn from(value: Number) -> Self {
        value.0
            .into_iter()
            .rev()
            .enumerate()
            .map(|(pos, val)| val.positional_value(pos))
            .sum::<i64>()
    }
}

impl From<i64> for Number {
    fn from(target: i64) -> Self {
        let max_exp = (1..).find(|i| 5_i64.pow(*i) >= target).unwrap() as usize;

        let mut digits = Vec::with_capacity(max_exp);
        let mut acc = 0_i64;

        for exp in (0..=max_exp).rev() {
            // find the digit that - if added to the number bulit so far - comes closest to 0
            let best_digit = ALL_DIGIS.into_iter().min_by_key(|digit|{
                            let digit_value = digit.positional_value(exp);
                            let diff = target.abs_diff(acc + digit_value);
                            diff
                        }).unwrap();
            digits.push(best_digit);
            acc += best_digit.positional_value(exp);
        }
        
        // remove leading zeroes
        while digits.get(0) == Some(&Digit::Zero) {
            digits.remove(0);
        }

        let num = Number(digits);
        if acc != target {
            panic!("Failed to parse value:{target} | best approximation:{num}");
        }
        num
    }
}

pub fn task1(input: Linewise<Number>) -> Result<Number, Infallible> {
    let mut sum: i64 = 0;
    common::for_input!(input, |num| { sum += i64::from(num) });
    Ok(sum.into())
}

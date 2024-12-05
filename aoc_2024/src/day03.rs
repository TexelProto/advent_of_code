use nom::branch::alt;
use nom::character::complete::anychar;
use nom::combinator::map;
use nom::multi::many_till;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

// Parse a number between 1 and 3 digits
fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |digit_str: &str| digit_str.parse::<u32>())(input)
}

// Parse a multiplication instruction
fn parse_mul_instruction(input: &str) -> IResult<&str, (u32, u32)> {
    delimited(
        tag("mul("),
        separated_pair(parse_number, char(','), parse_number),
        char(')'),
    )(input)
}
// Skip to the next potential mul instruction
fn skip_to_mul_instruction(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (_, mul)) = many_till(anychar, parse_mul_instruction)(input)?;
    Ok((input, mul))
}

fn parse_do_instruction(input: &str) -> IResult<&str, ()> {
    tag("do()")(input)?;
    Ok((input, ()))
}
fn parse_dont_instruction(input: &str) -> IResult<&str, ()> {
    tag("don't()")(input)?;
    Ok((input, ()))
}

fn skip_to_any_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, instruction)) = many_till(
        anychar,
        alt((
            map(parse_do_instruction, |_| Instruction::Do),
            map(parse_dont_instruction, |_| Instruction::Dont),
            map(parse_mul_instruction, |(a, b)| Instruction::Mul(a, b)),
        )),
    )(input)?;
    Ok((input, instruction))
}

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

pub fn task1(input: String) -> Result<u32, Error> {
    let mut input = input.as_str();
    let mut total = 0;
    loop {
        match skip_to_mul_instruction(input) {
            Ok((remain, (a, b))) => {
                total += a * b;
                input = remain;
            }
            Err(_) => break,
        }
    }

    Ok(total)
}

pub fn task2(input: String) -> Result<u32, Error> {
    let mut input = input.as_str();
    let mut total = 0;
    let mut active = true;
    loop {
        let i = skip_to_any_instruction(input);
        match i {
            Ok((_, Instruction::Do)) => {
                input = &input[4..];
                active = true;
            }
            Ok((_, Instruction::Dont)) => {
                input = &input[7..];
                active = false;
            }
            Ok((remain, Instruction::Mul(a, b))) => {
                if active {
                    total += a * b;
                }
                input = remain;
            }
            Err(_) => break,
        }
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        let result = task1(input);
        let val = result.unwrap();
        assert_eq!(val, 161);
    }
    #[test]
    fn test_task2() {
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        let result = task2(input);
        let val = result.unwrap();
        assert_eq!(val, 48);
    }
}

use std::{
    collections::HashMap,
    str::FromStr, num::ParseIntError,
};

use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Unknown operation '{0}'")]
    UnknownOperation(String),
}

#[derive(Debug, Clone, Copy)]
pub struct NamedExpression(u32, Expression);

impl FromStr for NamedExpression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = encode_name(&s[0..4]);
        let expr = Expression::from_str(&s[6..])?;
        Ok(Self(name, expr))    
    }
}

#[derive(Debug, Clone, Copy)]
enum Expression {
    Number(i64),
    Calculation { a: u32, b: u32, op: Operation },
}

impl Expression {
    fn eval(&self, map: &HashMap<u32, Expression>) -> i64 {
        match self {
            Expression::Number(i) => *i,
            Expression::Calculation { a, b, op } => {
                let a = map[a].eval(map);
                let b = map[b].eval(map);
                match op {
                    Operation::Add => a + b,
                    Operation::Sub => a - b,
                    Operation::Mul => a * b,
                    Operation::Div => a / b,
                }
            }
        }
    }
    fn rev_eval(&self, map: &HashMap<u32, Expression>, target: i64) -> i64 {
        match self {
            Expression::Number(i) => *i,
            Expression::Calculation { a, b, op } => {
                let a_expr = map[a];
                let b_expr = map[b];
                if *a == HUMN || a_expr.contains(map, HUMN) {
                    let set = b_expr.eval(map);
                    let new_target = match op {
                        Operation::Add => target - set,
                        Operation::Sub => target + set,
                        Operation::Mul => target / set,
                        Operation::Div => target * set,
                    };
                    if *a == HUMN || *b == HUMN {
                        new_target
                    } else {
                        a_expr.rev_eval(map, new_target)
                    }
                } else if *b == HUMN || b_expr.contains(map, HUMN) {
                    let set = a_expr.eval(map);
                    let new_target = match op {
                        Operation::Add => target - set,
                        Operation::Sub => set - target,
                        Operation::Mul => target / set,
                        Operation::Div => set / target,
                    };
                    if *a == HUMN || *b == HUMN {
                        new_target
                    } else {
                        b_expr.rev_eval(map, new_target)
                    }
                } else {
                    panic!("Neither node contained HUMN");
                }
            }
        }
    }

    fn contains(&self, map: &HashMap<u32, Expression>, name: u32) -> bool {
        match self {
            Expression::Number(_) => false,
            Expression::Calculation { a, b, op: _ } => {
                *a == name
                    || *b == name
                    || map[&a].contains(&map, name)
                    || map[&b].contains(&map, name)
            }
        }
    }
}

impl FromStr for Expression {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(char::is_numeric) {
            return Ok(Self::Number(i64::from_str(s)?));
        }

        let a = encode_name(&s[0..4]);
        let b = encode_name(&s[7..]);

        let op = match s.chars().nth(5).unwrap() {
            '+' => Operation::Add,
            '-' => Operation::Sub,
            '*' => Operation::Mul,
            '/' => Operation::Div,
            s => return Err(Error::UnknownOperation(s.to_string())),
        };

        Ok(Self::Calculation { a, b, op })
    }
}


#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

const HUMN: u32 = encode_name("humn");

const fn encode_name(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut num = 0_u32;
    num |= bytes[0] as u32;
    num |= (bytes[1] as u32) << 8;
    num |= (bytes[2] as u32) << 16;
    num |= (bytes[3] as u32) << 24;
    num
}

#[allow(dead_code)]
const fn decode_name(b: &u32) -> &str {
    let bytes = b as *const u32 as *const u8;
    unsafe {
        let bytes = std::slice::from_raw_parts(bytes, 4);
        std::str::from_utf8_unchecked(bytes)
    }
}

pub fn task1(expr: Linewise<NamedExpression>) -> Result<i64, Error> {
    let mut map = HashMap::new();
    common::for_input!(expr, |x| {map.insert(x.0, x.1)});
    Ok(map[&encode_name("root")].eval(&map))
}

pub fn task2(expr: Linewise<NamedExpression>) -> Result<i64, Error> {
    let mut map = HashMap::new();
    common::for_input!(expr, |x| {map.insert(x.0, x.1)});
    let root = match map.entry(encode_name("root")) {
        std::collections::hash_map::Entry::Occupied(occ) => occ.remove(),
        std::collections::hash_map::Entry::Vacant(_) => panic!("root not found"),
    };

    let (a, b) = match root {
        Expression::Number(_) => panic!("root must be calculation"),
        Expression::Calculation { a, b, op: _ } => (a, b),
    };

    let a = map[&a];
    let b = map[&b];
    let result = if a.contains(&map, HUMN) {
        a.rev_eval(&map, b.eval(&map))
    } else {
        b.rev_eval(&map, a.eval(&map))
    };

    Ok(result)
}

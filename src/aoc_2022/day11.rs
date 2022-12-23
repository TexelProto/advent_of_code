use std::{cell::RefCell, str::FromStr, fmt::Debug, num::ParseIntError};

use crate::input::Multiline;

#[derive(Debug)]
pub enum OpValue {
    Number(u64),
    Old,
}

impl FromStr for OpValue {
    type Err = Error ;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Self::Old)
        } else {
            let num = match s.parse::<u64>() {
                Ok(num) => num,
                Err(_) => return Err(Error::InvalidValue(s.to_owned())),
            };
            Ok(Self::Number(num))
        }
    }
}

impl OpValue {
    fn get_value(&self, old: u64) -> u64 {
        match self {
            Self::Number(i) => *i,
            Self::Old => old,
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    Add(OpValue),
    Multiply(OpValue),
}

impl Operation {
    fn apply_to(&self, old: u64) -> u64 {
        match self {
            Self::Add(value) => old + value.get_value(old),
            Self::Multiply(value) => old * value.get_value(old),
        }
    }
}

impl FromStr for Operation {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(' ').rev();

        let line = iter.next()
            .ok_or_else(|| Error::InvalidOperation(s.to_owned()))?;
        let num: OpValue = line.trim().parse()?;

        let line = iter.next()
            .ok_or_else(|| Error::InvalidOperation(s.to_owned()))?;
        let op = match line.trim() {
            "+" => Operation::Add(num),
            "*" => Operation::Multiply(num),
            _ => return Err(Error::InvalidOperation(s.to_owned())),
        };
        Ok(op)
    }
}

#[derive(Debug)]
pub struct Monkey {
    items: RefCell<Vec<u64>>,
    op: Operation,
    test: u64,
    true_target: usize,
    false_target: usize,
    inspected_items: usize,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        // Monkey 1:
        lines.next().ok_or(Error::LineGrouping(0))?;
        // Starting items: 54, 65, 75, 74
        let items = parse_items(lines.next().ok_or(Error::LineGrouping(1))?)?;
        // Operation: new = old + 3
        let op = Operation::from_str(lines.next().ok_or(Error::LineGrouping(2))?)?;
        // Test: divisible by 17
        let test : u64 = get_trailing_num(lines.next().ok_or(Error::LineGrouping(3))?)?;
        //   If true: throw to monkey 0
        let true_target = get_trailing_num(lines.next().ok_or(Error::LineGrouping(4))?)?;
        //   If false: throw to monkey 1
        let false_target = get_trailing_num(lines.next().ok_or(Error::LineGrouping(6))?)?;

        Ok(Monkey {
            items: RefCell::new(items),
            op,
            test: test.into(),
            true_target,
            false_target,
            inspected_items: 0,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from]ParseIntError),
    #[error("Errornious line grouping")]
    LineGrouping(usize),
    #[error("Unknown value '{0}'")]
    InvalidValue(String),
    #[error("Unknown operator '{0}'")]
    InvalidOperation(String),
}

fn parse_items(line: &str) -> Result<Vec<u64>, Error> {
    let mut iter = (line.trim()).split(' ');
    // "Starting "
    iter.next().unwrap();
    // "items: "
    iter.next().unwrap();

    let mut vec = Vec::new();
    while let Some(mut part) = iter.next() {
        part = part.trim_end_matches(',');
        vec.push((part).trim().parse::<u64>()?)
    }
    Ok(vec)
}

fn get_trailing_num<E: Debug, T: FromStr<Err = E>>(line: &str) -> Result<T,E> {
    let mut iter = line.trim().split(' ').rev();
    T::from_str(iter.next().unwrap())
}

fn run_turn(active_id: usize, monkeys: &mut Vec<Monkey>, common_factor: u64) {
    let active_monkey = &mut monkeys[active_id];
    active_monkey.inspected_items += active_monkey.items.borrow().len();

    let active_monkey = &monkeys[active_id];
    for mut item in active_monkey.items.borrow_mut().drain(..) {
        item = active_monkey.op.apply_to(item);
        // item = item / 3;

        let target = if item % active_monkey.test == 0 {
            active_monkey.true_target
        }
        else {
            active_monkey.false_target
        };

        item %= common_factor;
        monkeys[target].items.borrow_mut().push(item);
    }
}

fn run_round(monkeys: &mut Vec<Monkey>, common_factor: u64) {
    for i in 0..monkeys.len() {
        run_turn(i, monkeys, common_factor)
    }
}

pub fn task1(mut monkeys: Multiline<Monkey, 6, true>) -> Result<usize, Error> {
    let mut monkeys = monkeys.try_collect::<Vec<_>>()?;
    let common_factor = monkeys.iter().map(|m| m.test).product::<u64>();
    for _ in 0..20 {
        run_round(&mut monkeys, common_factor);
    }
    
    let mut inspecitons = monkeys.into_iter().map(|m| m.inspected_items).collect::<Vec<_>>();
    inspecitons.sort_unstable_by(|a,b| a.cmp(b).reverse());
    Ok(inspecitons[0] * inspecitons[1])
}

pub fn task2(mut monkeys: Multiline<Monkey, 6, true>) -> Result<usize, Error> {
    let mut monkeys = monkeys.try_collect::<Vec<_>>()?;
    let common_factor = monkeys.iter().map(|m| m.test).product::<u64>();
    for _ in 0..10_000 {
        run_round(&mut monkeys, common_factor);
    }
    
    let mut inspecitons = monkeys.into_iter().map(|m| m.inspected_items).collect::<Vec<_>>();
    inspecitons.sort_unstable_by(|a,b| a.cmp(b).reverse());
    Ok(inspecitons[0] * inspecitons[1])
}

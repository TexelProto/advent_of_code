use std::num::ParseIntError;
use common::input::Grouped;

pub fn task1(mut input: Grouped<u64>) -> Result<u64, ParseIntError> {
    let mut max = 0;
    while let Some(group) = input.next() {
        let group= group?;
        max = std::cmp::max(max, group.into_iter().sum());
    }
    Ok(max)
}
pub fn task2(mut input: Grouped<u64>) -> Result<u64, ParseIntError> {
    let mut values = Vec::new();
    while let Some(group) = input.next() {
        values.push(group?.into_iter().sum::<u64>());
    }
    values.sort_by(|a, b| a.cmp(b).reverse());
    Ok(values[..3].iter().sum())
}

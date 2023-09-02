use common::{input::CommaSeparated, iter_ext::try_collect};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

pub fn task1(input: CommaSeparated<u16>) -> Result<u32, Error> {
    let locations: Vec<_> = try_collect(input)?;

    let (max, min) = locations.iter()
        .cloned()
        .fold((u16::MIN, u16::MAX), |(acc_max, acc_min), num| {
            (acc_max.max(num), acc_min.min(num))
        });

    let mut min_fuel = u32::MAX;

    'outer: for x in min..=max {
        let mut fuel = 0_u32;
        for loc in locations.iter() {
            let distance = loc.abs_diff(x) as u32;
            fuel += distance;

            if fuel > min_fuel { 
                continue 'outer;
            }
        }
        min_fuel = fuel;
    }

    Ok(min_fuel)
}

pub fn task2(input: CommaSeparated<u16>) -> Result<u32, Error> {
    let locations: Vec<_> = try_collect(input)?;

    let (max, min) = locations.iter()
        .cloned()
        .fold((u16::MIN, u16::MAX), |(acc_max, acc_min), num| {
            (acc_max.max(num), acc_min.min(num))
        });

    let mut min_fuel = u32::MAX;

    'outer: for x in min..=max {
        let mut fuel = 0_u32;
        for loc in locations.iter() {
            let distance = loc.abs_diff(x) as u32;
            fuel += (1..=distance).sum::<u32>();

            if fuel > min_fuel { 
                continue 'outer;
            }
        }
        min_fuel = fuel;
    }

    Ok(min_fuel)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "16,1,2,0,4,2,7,1,2,14".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 37);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 168);
    }
}

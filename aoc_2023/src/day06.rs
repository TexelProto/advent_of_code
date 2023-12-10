use std::num::ParseIntError;
use common::input::Linewise;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to read line for {0}")]
    MissingLine(&'static str),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// Total distance follows the function:
/// ```ignore
/// f(t,x) = x * t(-x)
/// f(t,x) = xt - xx
/// ```
///
/// so to check if the final distance is greater than `m`
/// ```ignore
/// f(t,x,m) > 0
/// -x² + tx - m > 0
/// ```
///
/// to find the intersections with the minimum use the midnight formula
/// ```ignore
/// x = (-b +- sqrt(b² - 4ac)) / 2a
/// x = (-(t) +- sqrt((-t)² - 4(-1)(-m))) / 2(-1)
/// x = (-t +- sqrt(t² - 4m)) / -2
/// ```
pub fn task1(mut input: Linewise<String>) -> Result<u64, Error> {
    let times = input.next()
        .ok_or(Error::MissingLine("Time"))?
        .unwrap();
    let times = times[5..].split_ascii_whitespace().map(u64::from_str);
    let distances = input.next()
        .ok_or(Error::MissingLine("Distance"))?
        .unwrap();
    let distances = distances[9..].split_ascii_whitespace().map(u64::from_str);

    let mut result = 1;

    for (time, dist) in times.zip(distances) {
        let t = find_limits(time?, dist?);
        let len = t.1 - t.0 + 1;
        result *= len;
    };

    Ok(result)
}

fn find_limits(time: u64, min_dist: u64) -> (u64, u64) {
    let time = time as f64;
    let min_dist = min_dist as f64;

    let a = (-time + f64::sqrt(time.powi(2) - 4.0 * min_dist)) / -2.0;
    let b = (-time - f64::sqrt(time.powi(2) - 4.0 * min_dist)) / -2.0;

    let a = a.ceil() as u64;
    let b = b.floor() as u64;

    let min = a.min(b);
    let max = a.max(b);
    (min, max)
}

pub fn task2(mut input: Linewise<String>) -> Result<u64, Error> {
    let time = input.next()
        .ok_or(Error::MissingLine("Time"))?
        .unwrap()
        .replace(char::is_whitespace, "");
    let time = u64::from_str(&time[5..])?;

    let distance = input.next()
        .ok_or(Error::MissingLine("Distance"))?
        .unwrap()
        .replace(char::is_whitespace, "");
    let distance = u64::from_str(&distance[9..])?;

    let t = find_limits(time, distance);
    let result = t.1 - t.0 + 1;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = b"\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 288);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 32583852);
    }
}

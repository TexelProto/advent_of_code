use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to find any digit in line {0}")]
    MissingDigit(usize)
}

/// Find and parse the first and last digit in each line
pub fn task1(input: Linewise<String>) -> Result<u32, Error> {
    let mut total = 0u32;
    for (i,line) in input.enumerate() {
        let line = line.unwrap();

        let first = line.chars()
            .filter(|c| char::is_digit(*c, 10))
            .map(|c| c as u8 - b'0')
            .next()
            .ok_or(Error::MissingDigit(i))?;

        let last = line.chars().rev()
            .filter(|c| char::is_digit(*c, 10))
            .map(|c| c as u8 - b'0')
            .next()
            .ok_or(Error::MissingDigit(i))?;

        total += (first * 10 + last) as u32;
    }
    Ok(total)
}

fn try_read_number(s: &str) -> Option<u32> {
    let c = s.chars().next().unwrap();
    if char::is_digit(c, 10) {
        let value = (c as u8 - b'0') as u32;
        return Some(value);
    }

    // try to match the number literals if the string is long enough
    if s.len() < 3 { return None; }
    match &s[..3] {
        "one" => return Some(1),
        "two" => return Some(2),
        "six" => return Some(6),
        _ => {}
    }
    if s.len() < 4 { return None; }
    match &s[..4] {
        "four" => return Some(4),
        "five" => return Some(5),
        "nine" => return Some(9),
        "zero" => return Some(0),
        _ => {}
    }
    if s.len() < 5 { return None; }
    match &s[..5] {
        "three" => return Some(3),
        "seven" => return Some(7),
        "eight" => return Some(8),
        _ => {}
    }
    None
}

/// Find and parse the first and last number (digit or written) in each line
pub fn task2(input: Linewise<String>) -> Result<u32, Error> {
    let mut total = 0u32;
    for line in input {
        let line = line.unwrap();
        let mut cursor = 0;

        let first = loop {
            if let Some(num) = try_read_number(&line[cursor..]) {
                break num;
            }
            cursor += 1;
        };

        let mut cursor = line.len() - 1;
        let last = loop {
            if let Some(num) = try_read_number(&line[cursor..]) {
                break num;
            }
            cursor -= 1;
        };

        total += first * 10 + last;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(b"\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".as_slice());
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 142);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(b"\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen".as_slice());
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 281);
    }
}

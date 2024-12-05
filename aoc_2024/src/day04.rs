use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(input: Linewise<String>) -> Result<i32, Error> {
    let mut chars = vec![];
    for line in input {
        chars.push(line.unwrap().into_bytes())
    }

    let height = chars.len();
    let width = chars[0].len();
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            count_xmas_lines(&mut total, &chars, x, y);
        }
    }

    Ok(total)
}

fn count_xmas_lines(counter: &mut i32, chars: &Vec<Vec<u8>>, x: usize, y: usize) {
    is_xmas_line(counter, chars, x, y, -1, 1);
    is_xmas_line(counter, chars, x, y, 0, 1);
    is_xmas_line(counter, chars, x, y, 1, 1);
    is_xmas_line(counter, chars, x, y, -1, 0);
    // is_xmas_line(counter, chars, x, y, 0, 0);
    is_xmas_line(counter, chars, x, y, 1, 0);
    is_xmas_line(counter, chars, x, y, -1, -1);
    is_xmas_line(counter, chars, x, y, 0, -1);
    is_xmas_line(counter, chars, x, y, 1, -1);
}

fn is_xmas_line(
    counter: &mut i32,
    chars: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    x_step: isize,
    y_step: isize,
) -> Option<()> {
    if chars.get(y)?.get(x)? != &b'X' {
        return None;
    }
    let x = x.checked_add_signed(x_step)?;
    let y = y.checked_add_signed(y_step)?;
    if chars.get(y)?.get(x)? != &b'M' {
        return None;
    }
    let x = x.checked_add_signed(x_step)?;
    let y = y.checked_add_signed(y_step)?;
    if chars.get(y)?.get(x)? != &b'A' {
        return None;
    }
    let x = x.checked_add_signed(x_step)?;
    let y = y.checked_add_signed(y_step)?;
    if chars.get(y)?.get(x)? != &b'S' {
        return None;
    }
    *counter += 1;
    Some(())
}

pub fn task2(input: Linewise<String>) -> Result<i32, Error> {
    let mut chars = vec![];
    for line in input {
        chars.push(line.unwrap().into_bytes())
    }

    let height = chars.len();
    let width = chars[0].len();
    let mut total = 0;
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if is_xmas_cross(&chars, x, y) {
                total += 1;
            }
        }
    }

    Ok(total)
}

fn is_xmas_cross(chars: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    if chars[y][x] != b'A' {
        return false;
    }

    let mut pairs = 0;
    if chars[y - 1][x - 1] == b'M' && chars[y + 1][x + 1] == b'S' {
        pairs += 1;
    }
    if chars[y - 1][x + 1] == b'M' && chars[y + 1][x - 1] == b'S' {
        pairs += 1;
    }
    if chars[y + 1][x - 1] == b'M' && chars[y - 1][x + 1] == b'S' {
        pairs += 1;
    }
    if chars[y + 1][x + 1] == b'M' && chars[y - 1][x - 1] == b'S' {
        pairs += 1;
    }

    pairs == 2
}
#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 18);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 9);
    }
}

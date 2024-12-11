use std::ops::Range;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(input: String) -> Result<usize, Error> {
    debug_assert!(input.is_ascii());
    debug_assert!(input.chars().all(|c| char::is_ascii_digit(&c)));

    let ranges = parse_ranges(input);
    let mut values = create_values_from_ranges(&ranges);

    let mut read_cursor = values.len() - 1;
    let mut write_cursor = ranges[0].end;

    while read_cursor > write_cursor {
        let value = values[read_cursor];

        if value.is_none() {
            read_cursor -= 1;
            continue;
        }

        values[read_cursor] = None;
        values[write_cursor] = value;
        write_cursor += 1;
        read_cursor -= 1;

        if let Ok(index) = ranges.binary_search_by_key(&write_cursor, |range| range.start) {
            for range in &ranges[index..] {
                if range.contains(&write_cursor) {
                    write_cursor = range.end;
                }

                if range.start > write_cursor {
                    break;
                }
            }
        }
    }

    let total = values[..write_cursor]
        .into_iter()
        .enumerate()
        .filter_map(|(pos, value)| value.map(move |v| pos * v as usize))
        .sum();

    Ok(total)
}

fn create_values_from_ranges(ranges: &Vec<Range<usize>>) -> Vec<Option<u16>> {
    let mut values = vec![];
    let mut cursor = 0;
    for (i, range) in ranges.iter().enumerate() {
        values.extend(std::iter::repeat_n(None, range.start - cursor));
        values.extend(std::iter::repeat_n(Some(i as u16), range.len()));
        cursor = range.end;
    }
    values
}

fn parse_ranges(input: String) -> Vec<Range<usize>> {
    let mut position = 0;
    let mut ranges = Vec::with_capacity(input.len() / 2 + 1);
    let mut is_file = true;
    for value in input.as_bytes() {
        let value = (*value - b'0') as usize;
        if is_file && value > 0 {
            let end = position + value;
            ranges.push(position..end);
        }

        is_file = !is_file;
        position += value
    }
    ranges
}

pub fn task2(input: String) -> Result<usize, Error> {
    debug_assert!(input.is_ascii());
    debug_assert!(input.chars().all(|c| char::is_ascii_digit(&c)));

    let mut ranges = parse_ranges(input);
    let mut values = create_values_from_ranges(&ranges);

    let mut cursor = ranges.len() - 1;
    loop {
        let source = ranges[cursor].clone();
        let Some(index) = try_shift_range(&mut ranges, cursor) else {
            let Some(next) = cursor.checked_sub(1) else {
                break;
            };
            cursor = next;
            continue;
        };

        let destination = index..(index + source.len());
        let value = values[source.start];
        values[source].fill(None);
        values[destination].fill(value);
    }

    let total = values
        .into_iter()
        .enumerate()
        .filter_map(|(pos, value)| value.map(move |v| pos * v as usize))
        .sum();

    Ok(total)
}

fn try_shift_range(ranges: &mut Vec<Range<usize>>, index: usize) -> Option<usize> {
    let target = ranges[index].clone();
    let required_space = target.len();
    let mut found_prev_index = None;

    for (prev_index, pair) in ranges[..=index].windows(2).enumerate() {
        let [prev, after] = pair else { unreachable!() };

        let free = after.start - prev.end;
        if free < required_space {
            continue;
        }

        found_prev_index = Some(prev_index);
        break;
    }

    if let Some(prev_index) = found_prev_index {
        ranges.remove(index);

        let prev = ranges[prev_index].clone();
        let new_range = prev.end..(prev.end + target.len());
        ranges.insert(prev_index + 1, new_range);
        return Some(prev.end);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"2333133121414131402";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 1928);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 2858);
    }
}

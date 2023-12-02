use std::cmp::max;
use common::input::Linewise;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(input: Linewise<String>) -> Result<u32, Error> {
    let mut total = 0;
    for (i, line) in input.enumerate() {
        let line = line.unwrap();

        // skip the prefix of Game XY:
        let valid = line.split_once(':').unwrap()
            // split at the ; for the separate groups
            .1.split(';')
            .all(|group| {
                // break the group into individual elements
                group.split(',').all(|element| {
                    let (num, color) = element.trim().split_once(' ')
                        .expect("Encountered malformed element");
                    let num = u8::from_str(num).unwrap();
                    let max = match color {
                        "red" => 12,
                        "green" => 13,
                        "blue" => 14,
                        _ => panic!("Invalid color {color}")
                    };

                    num <= max
                })
            });

        if valid {
            total += (i + 1) as u32;
        }
    }
    Ok(total)
}

pub fn task2(input: Linewise<String>) -> Result<u32, Error> {
    let mut total = 0;
    for line in input {
        let line = line.unwrap();

        // skip the prefix of Game XY:
        let groups = line.split_once(':').unwrap().1;

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for group in groups.split(';') {
            for element in group.split(',') {
                let (num, color) = element.trim().split_once(' ')
                    .expect("Encountered malformed element");
                let num = u32::from_str(num).unwrap();
                match color {
                    "red" => max_red = max(max_red, num),
                    "green" => max_green = max(max_green, num),
                    "blue" => max_blue = max(max_blue, num),
                    _ => panic!("Invalid color {color}")
                };
            }
        }
        let power = max_red * max_green * max_blue;
        total += power;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = b"\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 8);
    }

    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 2286);
    }
}

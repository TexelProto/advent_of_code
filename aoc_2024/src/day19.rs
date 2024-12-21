use common::input::{LineSeparated, Linewise};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1<'a>(input: LineSeparated<'a, String, Linewise<'a, String>>) -> Result<u32, Error> {
    let (pattern_str, goals) = input.into_inner();
    let patterns: Vec<_> = pattern_str.split(", ").map(|s| s.as_bytes()).collect();

    let mut possible = 0;
    for goal in goals {
        if try_match(&patterns, &goal.unwrap().as_bytes()) {
            possible += 1;
        }
    }

    Ok(possible)
}

fn try_match(patterns: &Vec<&[u8]>, goal: &[u8]) -> bool {
    for pattern in patterns {
        if goal.starts_with(pattern) == false {
            continue;
        }

        if goal.len() == pattern.len() {
            return true;
        }

        let remain = &goal[pattern.len()..];
        if try_match(patterns, remain) {
            return true;
        }
    }
    false
}

pub fn task2<'a>(input: LineSeparated<'a, String, Linewise<'a, String>>) -> Result<u64, Error> {
    let (pattern_str, goals) = input.into_inner();
    let patterns: Vec<_> = pattern_str.split(", ").map(|s| s.as_bytes()).collect();

    let mut possible = 0;
    for goal in goals {
        possible += count_matches(&patterns, &goal.unwrap().as_bytes());
    }

    Ok(possible)
}
fn count_matches(patterns: &Vec<&[u8]>, goal: &[u8]) -> u64 {
    let n = goal.len();
    // [i] will store the number of ways to construct goal[0..i]
    let mut paths = vec![0; n + 1];
    paths[0] = 1;

    // Iterate over each position in the goal
    for cursor in 0..=n {
        // Check each pattern to see if it fits at the current position
        for pattern in patterns {
            if pattern.len() > cursor {
                continue;
            }

            let start = cursor - pattern.len();
            if &goal[start..cursor] == *pattern {
                paths[cursor] += paths[start];
            }
        }
    }

    paths[n] // Number of ways to construct the full goal string
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 6);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 16);
    }
}

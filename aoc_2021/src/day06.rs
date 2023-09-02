use common::input::CommaSeparated;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError)
}


pub fn task1(input: CommaSeparated<usize>) -> Result<u64, Error> {
    let mut state = [0; 9];

    for i in input {
        state[i?] += 1;
    }

    for _ in 0..80 {
        let mut new_state = [0; 9];

        // copy each bucket one day forwards
        new_state[0..8].copy_from_slice(&state[1..9]);
        // add all newly born fish
        new_state[8] = state[0];
        // reset all the fish that reproduced
        new_state[6] += state[0];

        // update the state for the next iteration
        state = new_state;
    }

    Ok(state.into_iter().sum())
}

pub fn task2(input: CommaSeparated<usize>) -> Result<u64, Error> {
    let mut state = [0; 9];

    for i in input {
        state[i?] += 1;
    }

    for _ in 0..256 {
        let mut new_state = [0; 9];

        // copy each bucket one day forwards
        new_state[0..8].copy_from_slice(&state[1..9]);
        // add all newly born fish
        new_state[8] = state[0];
        // reset all the fish that reproduced
        new_state[6] += state[0];

        // update the state for the next iteration
        state = new_state;
    }

    Ok(state.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "3,4,3,1,2".as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 5934);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 26984457539_u64);
    }
}

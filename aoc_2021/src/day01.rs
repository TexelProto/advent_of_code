use common::{input::Linewise, iter_ext::try_collect};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

pub fn task1(input: Linewise<u32>) -> Result<usize, Error> {
    let vec: Vec<_> = try_collect(input)?;
    let count = vec.windows(2).filter(|w| w[1] > w[0]).count();
    Ok(count)
}

pub fn task2(input: Linewise<u32>) -> Result<usize, Error> {
    let vec: Vec<_> = try_collect(input)?;
    let windows: Vec<_> = vec.windows(3)
        .map(|w| w.iter().sum::<u32>())
        .collect();
    let count = windows.windows(2).filter(|w| w[1] > w[0]).count();
    Ok(count)
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = "199
200
208
210
200
207
240
269
260
263"
    .as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 7);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 5);
    }
}

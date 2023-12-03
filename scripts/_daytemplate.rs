#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn task1(input: ()) -> Result<i32, Error> {
    todo!()
}

pub fn task2(input: ()) -> Result<i32, Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use common::input::Input;
    use super::*;

    const INPUT: &[u8] = b"\
";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 0);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 0);
    }
}

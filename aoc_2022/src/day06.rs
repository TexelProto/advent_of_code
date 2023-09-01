#[derive(Debug, thiserror::Error)]
#[error("No distinct section was found")]
pub struct NoDistinct;

fn all_distinct(chars: &[u8]) -> bool {
    for i in 0..chars.len() {
        for j in i + 1..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    return true;
}

fn find_distinct_window(chars: &[u8], size: usize) -> Option<usize> {
    chars.windows(size)
        .enumerate()
        .filter(|t| all_distinct(t.1))
        .next()
        .map(|t| t.0)
}

pub fn task1(input: String) -> Result<usize, NoDistinct> {
    let vec = input.as_str().as_bytes();
    match find_distinct_window(vec, 4) {
        None => Err(NoDistinct),
        Some(x) => Ok(x + 4),
    }
}
pub fn task2(input: String) -> Result<usize, NoDistinct> {
    let vec = input.as_str().as_bytes();
    match find_distinct_window(vec, 14) {
        None => Err(NoDistinct),
        Some(x) => Ok(x + 14),
    }
}

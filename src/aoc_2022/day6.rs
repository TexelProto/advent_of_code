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

pub fn task1(mut input: String) {
    let vec = unsafe { input.as_mut_vec() };
        let result = vec
            .windows(4)
            .enumerate()
            .filter(|t| all_distinct(t.1))
            .next()
            .unwrap();

        println!("Found after reading {} chars", result.0 + 4)
    }
    pub fn task2(mut input: String) {
        let vec = unsafe { input.as_mut_vec() };
        let result = vec
            .windows(14)
            .enumerate()
            .filter(|t| all_distinct(t.1))
            .next()
            .unwrap();

        println!("Found after reading {} chars", result.0 + 14)
    }

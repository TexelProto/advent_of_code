fn get_calories(input: String) -> Vec<usize> {
    let mut values = Vec::new();
    let mut lines = input.lines().peekable().into_iter();
    loop {
        let mut sum: usize = 0;
        while let Some(line) = lines.next() {
            if line.trim().len() == 0 {
                break;
            }

            let i = usize::from_str_radix(line, 10).unwrap();
            sum += i;
        }

        values.push(sum);

        if lines.peek().is_none() {
            break;
        }
    }
    values
}

    pub fn task1(input: String) {
        let values = get_calories(input);
        println!("Max calories: {}", values.into_iter().max().unwrap());
    }
    pub fn task2(input: String) {
        let mut values = get_calories(input);
        values.sort_by(|a, b| a.cmp(b).reverse());
        let top3 = &values[..3];
        println!("Max calories: {:?}", top3);
        println!("Max sum: {}", top3.into_iter().sum::<usize>());
    
}

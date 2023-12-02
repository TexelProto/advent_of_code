use common::{iter_ext::TryIterator, input::Linewise};

pub fn task1(input: Linewise<i128>) -> Result<i128, std::num::ParseIntError> {
    let mut nums = vec![];

    common::for_input!(input, |i| {
        nums.push((nums.len(), i));
    });

    shuffle(&mut nums);

    let len = nums.len();
    let zero_index = nums.iter().position(|(_, num)| *num == 0).unwrap();
    let (_, a) = nums[(zero_index + 1000) % len];
    let (_, b) = nums[(zero_index + 2000) % len];
    let (_, c) = nums[(zero_index + 3000) % len];

    Ok(a + b + c)
}

pub fn task2(input: Linewise<i128>) -> Result<i128, std::num::ParseIntError> {
    const DECRYPT_KEY: i128 = 811589153;

    let original: Vec<_> = input.try_collect2()?;
    let len = original.len();

    let mut shift: Vec<_> = original
        .iter()
        .cloned()
        .map(|i| i * DECRYPT_KEY)
        .enumerate()
        .collect();

    for _ in 0..10 {
        shuffle(&mut shift);
    }

    let zero_index = shift.iter().position(|(_, num)| *num == 0).unwrap();
    let (_, a) = shift[(zero_index + 1000) % len];
    let (_, b) = shift[(zero_index + 2000) % len];
    let (_, c) = shift[(zero_index + 3000) % len];
    let sum = a + b + c;
    Ok(sum)
}

fn shuffle(nums: &mut Vec<(usize, i128)>) {
    for i in 0..nums.len() {
        shift_num(nums, i);
    }
}

fn shift_num(nums: &mut Vec<(usize, i128)>, i: usize) {
    let from = nums.iter().position(move |(idx, _)| *idx == i).unwrap();
    let (_, num) = nums[from];

    if num == 0 {
        return;
    }

    let len = nums.len() as i128;
    let mut to = from as i128 + num;
    // when the index is below 0, we need to wrap around
    if to < 0 {
        // calculate the number of loops (+1 for truncation in int division)
        // excessive looping will be fixed by the modulo later
        let loops = to.abs() / (len - 1) + 1;
        to += loops * (len - 1);
    }

    to %= len - 1;
    let to = to as usize;

    if from < to {
        nums[from..=to].rotate_left(1);
    } else {
        nums[to..=from].rotate_right(1);
    }
}

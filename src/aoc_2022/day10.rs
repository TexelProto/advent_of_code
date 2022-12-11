use std::{str::FromStr, iter::Iterator};

#[derive(Debug)]
enum Change {
    ChangeX(isize),
    Noop,
}

fn parse<'a>(input: &'a str) -> impl 'a + Iterator<Item = Change> {
    input.lines()
        .flat_map(|l| {
            if l == "noop" {
                vec![Change::Noop]
            } else {
                let (_, diff) = l.split_at(4);
                let diff = isize::from_str(diff.trim()).unwrap();
                vec![
                    Change::Noop,
                    Change::ChangeX(diff),
                ]
            }
        })
}

pub fn task1(input: String) {
    let mut acc = 1_isize;

    let mut samples = Vec::new();
    let changes = parse(&input).enumerate();
    for (i, change) in changes {
        let i = i as isize;
        match dbg!(change) {
            Change::ChangeX(diff) => acc += diff,
            Change::Noop => {}
        }

        let step = i + 1;
        if (step - 20) % 40 == 0{
            dbg!(acc,step, acc * step);
            samples.push(acc * step);
        }
    }

    println!("total: {}", samples.into_iter().sum::<isize>())
}
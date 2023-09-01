use std::{fmt::Display, io::BufReader};

fn get_input<T, E>(prompt: &str) -> T
where
    T: std::str::FromStr<Err = E>,
    E: std::error::Error,
{
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    println!("{}", prompt);

    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("Failed to read line");

        match T::from_str(buffer.as_str().trim()) {
            Ok(result) => break result,
            Err(err) => println!("{}", err),
        }
    }
}

fn select_from_list<T, I, F, S>(elements: I, mut sort: F) -> T
where
    T: Display + Clone,
    I: IntoIterator<Item = T>,
    F: FnMut(&T) -> S,
    S: Ord,
{
    let mut days = elements.into_iter().collect::<Vec<_>>();
    days.sort_by(|a, b| sort(a).cmp(&sort(b)));
    days.iter()
        .enumerate()
        .for_each(|(i, t)| println!("({:2}) {}", i, t));

    loop {
        let index: usize = get_input("Enter number:");
        match days.get(index) {
            Some(t) => break t.clone(),
            None => println!("Invalid number"),
        }
    }
}

pub fn run() -> Result<(), std::io::Error> {
    let year = select_from_list(crate::YEARS, |y| y.name);
    let day = select_from_list(year.days, |d| d.name);
    let task = select_from_list(day.tasks, |t| t.name);

    let dir = std::env::current_dir()?;
    let full = loop {
        let prompt = format!("Input file location (relative to {:?}):", dir);
        let path: std::path::PathBuf = get_input(&prompt);
        match path.canonicalize() {
            Ok(full) => break full,
            Err(e) => println!("No file found at: {:?}", e),
        };
    };

    let file = std::fs::File::open(&full).unwrap();
    let mut buf = BufReader::new(file);
    let result = task.run(&mut buf);

    println!("{}", crate::format_simple(result));

    println!("Press enter to exit...");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}

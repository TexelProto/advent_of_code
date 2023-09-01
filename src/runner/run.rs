use std::{path::{Path, PathBuf}, io::BufReader};

#[derive(Debug, thiserror::Error)]
pub enum PartNotFound {
    #[error("Failed to find year {0}")]
    Year(String),
    #[error("Failed to find day {0}")]
    Day(String),
    #[error("Failed to find task {0}")]
    Task(String),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PartNotFound(#[from] PartNotFound),
    #[error("Failed to find file '{0:?}'")]
    FileNotFound(PathBuf),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub fn run(
    year: &str,
    day: &str,
    task: &str,
    input: &Path,
    _output: Option<&Path>,
) -> Result<(), Error> {
    let y = crate::YEARS
        .iter()
        .filter(|y| y.name == year)
        .next();

    let year = match y {
        Some(y) => y,
        None => return Err(PartNotFound::Year(year.to_owned()).into()),
    };

    let d = year.days.into_iter().filter(|d| d.name == day).next();

    let day = match d {
        Some(d) => d,
        None => return Err(PartNotFound::Day(day.to_owned()).into()),
    };

    let t = day.tasks.into_iter().filter(|d| d.name == task).next();

    let task = match t {
        Some(d) => d,
        None => return Err(PartNotFound::Task(task.to_owned()).into()),
    };

    let file = std::fs::File::open(&input)
        .map_err(move |_| Error::FileNotFound(input.to_path_buf()))?;

    let mut buf = BufReader::new(file);

    let time = std::time::Instant::now();
    let result = task.run(&mut buf);
    let elapsed = time.elapsed();

    println!("{}", crate::format_detailed(result, task, elapsed));
    Ok(())
}

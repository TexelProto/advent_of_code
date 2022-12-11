use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum PartNotFound {
    Year(String),
    Day(String),
    Task(String),
}

impl Display for PartNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Year(s) => f.write_fmt(format_args!("Failed to find year '{}'", s)),
            Self::Day(s) => f.write_fmt(format_args!("Failed to find day '{}'", s)),
            Self::Task(s) => f.write_fmt(format_args!("Failed to find task '{}'", s)),
        }
    }
}

#[derive(Debug)]
pub struct FileNotFound(PathBuf);

impl Display for FileNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Failed to find file '{:?}'", self.0))
    }
}

crate::error_wrapper! {
    Error {
        PartNotFound(PartNotFound),
        FileNotFound(FileNotFound),
    }
}

pub fn run(
    year: &str,
    day: &str,
    task: &str,
    input: &Path,
    _output: Option<&Path>,
) -> Result<(), Error> {
    let y = advent_of_code::get_years()
        .filter(|y| y.name() == year)
        .next();

    let year = match y {
        Some(y) => y,
        None => Err(PartNotFound::Year(year.to_owned()))?,
    };

    let d = year.days().into_iter().filter(|d| d.name() == day).next();

    let day = match d {
        Some(d) => d,
        None => Err(PartNotFound::Day(day.to_owned()))?,
    };

    let t = day.tasks().into_iter().filter(|d| d.name() == task).next();

    let task = match t {
        Some(d) => d,
        None => Err(PartNotFound::Task(task.to_owned()))?,
    };

    let input = std::fs::read_to_string(input).map_err(|_| FileNotFound(input.to_owned()))?;

    task.run(input);

    Ok(())
}

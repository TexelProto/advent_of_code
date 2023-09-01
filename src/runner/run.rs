use std::{path::PathBuf, io::BufReader};

#[derive(Debug, clap_derive::Parser)]
pub struct Args {
    #[clap(help = "The year of the task to be run. (i.e. aoc_2022)")]
    year: String,
    #[clap(help = "The day of the task to be run. (i.e. day01)")]
    day: String,
    #[clap(help = "The name of the task to be run. (i.e. task1)")]
    task: String,
    #[clap(short, long, help = "The path to the input file. If omitted it will be assumed to './inputs/YEAR/DAY.txt'.")]
    input: Option<PathBuf>,
    #[clap(short, long, help = "The path to the output file. If omitted the result will be written to stdout.")]
    _output: Option<PathBuf>,
}

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

pub fn run(args: Args) -> Result<(), Error> {
    let y = crate::YEARS
        .iter()
        .filter(|y| y.name == args.year)
        .next();

    let year = match y {
        Some(y) => y,
        None => return Err(PartNotFound::Year(args.year.to_owned()).into()),
    };

    let d = year.days.into_iter().filter(|d| d.name == args.day).next();

    let day = match d {
        Some(d) => d,
        None => return Err(PartNotFound::Day(args.day.to_owned()).into()),
    };

    let t = day.tasks.into_iter().filter(|d| d.name == args.task).next();

    let task = match t {
        Some(d) => d,
        None => return Err(PartNotFound::Task(args.task.to_owned()).into()),
    };

    let input_path = match args.input {
        Some(p) => p,
        None => PathBuf::from(format!(
            "inputs/{}/{}.txt", args.year, args.day
        ))
    };

    let file = std::fs::File::open(&input_path)
        .map_err(move |_| Error::FileNotFound(input_path))?;

    let mut buf = BufReader::new(file);

    let time = std::time::Instant::now();
    let result = task.run(&mut buf);
    let elapsed = time.elapsed();

    println!("{}", crate::format_detailed(result, task, elapsed));
    Ok(())
}

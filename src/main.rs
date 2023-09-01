#![deny(private_in_public)]

use clap::{value_parser, Arg};
use std::borrow::Cow;
use std::time::Duration;
use std::path::PathBuf;

mod runner {
    pub mod all;
    pub mod cli;
    pub mod run;
    pub mod tui;
}

static YEARS: &[&common::Year] = &[
    &aoc_2022::YEAR,
];

fn main() -> Result<(), anyhow::Error> {
    use clap::{command, Command};
    let matches = command!()
        .subcommand(
            Command::new("tui")
                .about("Renders a terminal user interface for interactive execution of tasks.")
        )
        .subcommand(
            Command::new("all")
                .about("Runs all tasks in parallel trying to solve all problems as fast as possible.")
        )
        .subcommand(
            Command::new("run")
                .about("Runs the specified task and returns.")
                .arg(Arg::new("year")
                    .help("The year of the task to be run. (i.e. aoc_2022)")
                    .value_parser(value_parser!(String)))
                .arg(Arg::new("day")
                    .help("The day of the task to be run. (i.e. day1)")
                    .value_parser(value_parser!(String)))
                .arg(Arg::new("task")
                    .help("The name of the task to be run. (i.e. task1)")
                    .value_parser(value_parser!(String)))
                .arg(Arg::new("input")
                    .help("The path to the input file. If omitted the result will be printed to stdout.")
                    .value_parser(value_parser!(PathBuf)))
                .arg(Arg::new("output")
                    .help("The path to the output file. If omitted the result will be printed to stdout.")
                    .value_parser(value_parser!(PathBuf)))
        )
        .get_matches();
    if let Some(_tui) = matches.subcommand_matches("tui") {
        runner::tui::run()?;
    } else if let Some(_all) = matches.subcommand_matches("all") {
        runner::all::run()?;
    } else if let Some(run) = matches.subcommand_matches("run") {
        let year = run.get_one::<String>("year").unwrap();
        let day = run.get_one::<String>("day").unwrap();
        let task = run.get_one::<String>("task").unwrap();
        let input = match run.get_one::<PathBuf>("input") {
            None => {
                let mut path = PathBuf::from_iter(["inputs", year, day]);
                path.set_extension("txt");
                Cow::Owned(path)
            }
            Some(path) => Cow::Borrowed(path),
        };
        let output = run.get_one::<PathBuf>("output");
        let output_path = output.map(|p| p.as_path());
        runner::run::run(year, day, task, &input, output_path)?;
    } else {
        runner::cli::run()?;
    }
    Ok(())
}

fn format_simple(res: Result<String, String>) -> String {
    let (status, message) = match res {
        Ok(ok) => ("OK ", ok),
        Err(e) => ("ERR", e),
    };

    format!("{} {}", status, message)
}

fn format_duration(duration: Duration) -> String {
    let s = duration.as_secs();
    let ms = duration.subsec_millis();
    let ys = duration.subsec_micros() % 1000;
    let ns = duration.subsec_nanos() % 1000;
    if duration.as_secs() > 0 {
        format!("{s}.{ms}s")
    } else if ms > 0 {
        format!("{ms}.{ys}ms")
    } else if ys > 0 {
        format!("{ys}.{ns}ys")
    } else {
        format!("{ns}ns")
    }
}

fn format_detailed(res: Result<String, String>, task: &common::Task, duration: Duration) -> String {
    let (status, message) = match res {
        Ok(ok) => ("OK ", ok),
        Err(e) => ("ERR", e),
    };

    let duration = format_duration(duration);
    let name = task.full_name;

    format!("{status} [{duration:9}] {name:26} {message}")
}
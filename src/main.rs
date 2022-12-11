#![deny(private_in_public)]

use std::{error::Error, path::PathBuf};

use clap::{value_parser, Arg};

mod runner {
    pub mod all;
    pub mod cli;
    pub mod run;
    pub mod tui;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::{arg, command, Command};
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
        runner::tui::run().map_err(box_error)
    } else if let Some(_all) = matches.subcommand_matches("all") {
        runner::all::run().map_err(box_error)
    } else if let Some(run) = matches.subcommand_matches("run") {
        let year = run.get_one::<String>("year").unwrap();
        let day = run.get_one::<String>("day").unwrap();
        let task = run.get_one::<String>("task").unwrap();
        let input = run.get_one::<PathBuf>("input").unwrap();
        let output = run.get_one::<PathBuf>("output");
        let output_path = output.map(|p| p.as_path());
        runner::run::run(year, day, task, input, output_path).map_err(box_error)
    } else {
        runner::cli::run().map_err(box_error)
    }
}

fn box_error<E: 'static + Error>(e: E) -> Box<dyn 'static + Error> {
    Box::new(e)
}

#[macro_export]
macro_rules! error_wrapper {
    (
        $name:ident {
            $( $var:ident($ty:path), )*
        }
    ) => {
        #[derive(Debug)]
        pub enum $name {
            $( $var($ty), )*
        }

        impl std::error::Error for $name {}

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self:: $var (val) => std::fmt::Display::fmt(&val, f),
                    )*
                }
            }
        }

        $(
            impl From<$ty> for $name {
                fn from(value: $ty) -> Self {
                    Self:: $var (value)
                }
            }
        )*
    };
}

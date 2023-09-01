#![deny(private_in_public)]

use std::time::Duration;

mod runner {
    #[cfg(feature = "parallel")]
    pub mod all;
    pub mod cli;
    pub mod run;
    #[cfg(feature = "interactive")]
    pub mod tui;
}

static YEARS: &[&common::Year] = &[
    &aoc_2022::YEAR,
];

#[derive(Debug, clap_derive::Parser)]
pub enum Args {
    #[cfg(feature = "parallel")]
    #[command(about = "Runs all tasks in parallel printing all the results.")]
    All(runner::all::Args),
    #[command(about = "Runs the specified task and returns.")]
    Run(runner::run::Args),
    #[cfg(feature = "interactive")]
    #[command(about = "Renders a terminal user interface for interactive execution of tasks.")]
    Tui(runner::tui::Args),
}

fn main() -> Result<(), anyhow::Error> {
    if dbg!(std::env::args().count()) != 1 {
        let args = <Args as clap::Parser>::parse();
        match args {
            Args::All(args) => runner::all::run(args)?,
            Args::Run(args) => runner::run::run(args)?,
            Args::Tui(args) => runner::tui::run(args)?,
        };
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
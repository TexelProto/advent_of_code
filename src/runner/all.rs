use rayon::prelude::*;
use std::convert::Infallible;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

pub fn run() -> Result<(), Infallible> {
    let tasks = advent_of_code::get_years()
        .flat_map(|y| {
            y.days().iter().flat_map(|d| {
                let mut path = PathBuf::from_iter(["inputs", y.name(), d.name()]);
                path.set_extension("txt");
                d.tasks().iter().map(move |t| (t, path.clone()))
            })
        })
        .collect::<Vec<_>>();
    let stdout = std::io::stdout();
    let total_time = tasks
        .into_par_iter()
        .map(|t| {
            let (task, path) = t;
            let time = std::time::Instant::now();
            let result = task.run(path.as_path());
            let elapsed = time.elapsed();

            let _ = stdout.lock().write_fmt(format_args!(
                "Task {} finished in {:-9?}: {:?}\n",
                task.full_name(),
                elapsed,
                result
            ));

            elapsed
        })
        .sum::<Duration>();

    println!("Finished!\ntotal time: {:?}", total_time);
    Ok(())
}

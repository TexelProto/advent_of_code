use std::convert::Infallible;
use std::io::Write;
use std::time::Duration;
use rayon::prelude::*;

pub fn run() -> Result<(), Infallible> {
    let tasks = advent_of_code::get_tasks().collect::<Vec<_>>();
    let stdout = std::io::stdout();
    let total_time = tasks.into_par_iter().map(|t| {
        let time = std::time::Instant::now();
        let result = t.run("".to_owned());
        let elapsed = time.elapsed();

        let _ = stdout.lock().write_fmt(format_args!("Task {} finished in {:?}: {:?}\n", t.full_name(), elapsed, result));

        elapsed
    }).sum::<Duration>();
    
    println!("Finished!\ntotal time: {:?}", total_time);
    Ok(())
}

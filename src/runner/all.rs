mod threading;

use std::convert::Infallible;

use threading::*;

pub fn run() -> Result<(), Infallible> {
    let _pool = ThreadPool::new();
    for _day in advent_of_code::get_tasks() {
        unimplemented!();
    }
    Ok(())
}

#![feature(iterator_try_collect)]

use std::fmt::Debug;
use std::path::Path;

pub mod input;
pub mod common {
    pub mod pathfinding;
    pub mod iter_try;
}

#[derive(Debug)]
pub struct Year {
    name: &'static str,
    days: &'static [Day],
}
impl Year {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn days(&self) -> &'static [Day] {
        self.days
    }
}
impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

#[derive(Debug)]
pub struct Day {
    name: &'static str,
    tasks: &'static [Task],
}
impl Day {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn tasks(&self) -> &'static [Task] {
        self.tasks
    }
}
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

pub struct Task {
    full_name: &'static str,
    name: &'static str,
    func: &'static (dyn Sync + Fn(&Path) -> String),
}
impl Task {
    pub fn full_name(&self) -> &'static str {
        self.full_name
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn run(&self, input: &Path) -> String {
        (self.func)(input)
    }
}
impl Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Task")
            .field("full_name", &self.full_name)
            .finish()
    }
}
impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

fn parse_input<T: input::Input>(read: input::Reader) -> Result<T, T::Error> {
    T::parse(read)
}

macro_rules! decl_years {
    (
        $($year:ident {
            $($day:ident {
                $(
                    $(#[$attr:ident])*
                    $task:ident;
                )*
            })*
        })*
    ) => {
        $(pub mod $year {
            $( pub mod $day; )*
        })*
        const YEARS: &[Year] = &[
            $(Year {
                name: stringify!($year),
                days: &[
                    $(Day {
                        name: stringify!($day),
                        tasks: &[
                            $(Task {
                                full_name: stringify!($year::$day::$task),
                                name: stringify!($task),
                                func: & |path| {
                                    const NAME: &str = stringify!($year::$day::$task);
                                    let file = match std::fs::File::open(path) {
                                        Ok(file) => file,
                                        Err(e) => {
                                            return format!("ERR {:?}", e);
                                        }
                                    };
                                    let buf_file = std::io::BufReader::new(file);
                                    let result: Result<_,_> = parse_input(buf_file);
                                    let input = match result {
                                        Ok(input) => input,
                                        Err(e) => {
                                            return format!("ERR {:?}", e);
                                        }
                                    };
                                    let result: Result<_, _> = $year :: $day :: $task (input);
                                    match result {
                                        Ok(ok) => format!("OK {}", ok),
                                        Err(e) => format!("ERR {:?}", e),
                                    }
                                }
                            },)*
                        ]
                    },)*
                ]
            },)*
        ];
    };
}

#[macro_export]
macro_rules! for_input {
    ($iter:ident, |$ele:ident| $body:tt) => {
        let mut m_iter = $iter;
        while let Some($ele) = Iterator::next(&mut m_iter) {
            let $ele = $ele?;
            $body;
        }
    };
}

decl_years! {
    aoc_2022 {
        day01 {task1;task2;}
        day02 {task1;task2;}
        day03 {task1;task2;}
        day04 {task1;task2;}
        day05 {task1;task2;}
        day06 {task1;task2;}
        day07 {task1;task2;}
        day08 {task1;task2;}
        day09 {task1;task2;}
        day11 {task1;task2;}
        day12 {task1;task2;}
        day13 {task1;task2;}
        day14 {task1;task2;}
        day17 {task1;task2;}
        // day18 {task1;task2;}
        // day20 {task1;task2;}
        // day21 {task1;task2;}
        // day22 {task1;task2;}
        // day23 {task1;task2;}
    }
}

pub fn get_years() -> impl Iterator<Item = &'static Year> {
    YEARS.into_iter()
}
pub fn get_days() -> impl Iterator<Item = &'static Day> {
    get_years().flat_map(|y| y.days)
}
pub fn get_tasks() -> impl Iterator<Item = &'static Task> {
    get_days().flat_map(|y| y.tasks)
}

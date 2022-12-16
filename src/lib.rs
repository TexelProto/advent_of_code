use std::fmt::Debug;
use std::io::BufRead;

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
    func: &'static (dyn Fn(String) + Sync),
}
impl Task {
    pub fn full_name(&self) -> &'static str {
        self.full_name
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn run(&self, input: String) {
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
        $(
            pub mod $year {
                $( pub mod $day; )*
            }
        )*
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
                                func: & $year :: $day :: $task
                            },)*
                        ]
                    },)*
                ]
            },)*
        ];
    };
}

trait Input: Sized {
    type Error: std::error::Error;
    fn parse(read: impl BufRead) -> Result<Self, Self::Error>;
}

impl Input for String {
    type Error = std::io::Error;
    fn parse(mut read: impl BufRead) -> Result<Self, Self::Error> {
        let mut s = String::new();
        read.read_to_string(&mut s)?;
        Ok(s)
    }
}

decl_years! {
    aoc_2022 {
        day1 {task1;task2;}
        day2 {task1;task2;}
        day3 {task1;task2;}
        day4 {task1;task2;}
        day5 {task1;task2;}
        day6 {task1;task2;}
        day7 {task1;task2;}
        day8 {task1;task2;}
        day9 {task1;task2;}
        day14 {task1;task2;}
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

use std::fmt::Debug;
use std::io::BufRead;

pub mod common{
    pub mod iter_ext;
    pub mod pathfinding;
}
mod input;

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

type TaskFn = dyn Sync + Fn(&mut dyn BufRead) -> Result<String, String>;
pub struct Task {
    full_name: &'static str,
    name: &'static str,
    func: &'static TaskFn,
}
impl Task {
    pub fn full_name(&self) -> &'static str {
        self.full_name
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn run(&self, input: &mut impl BufRead) -> Result<String, String> {
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

fn parse_input<'a, T, R>(read: &'a mut R) -> Result<T, T::Error> 
    where T: input::Input<'a>, R: 'a + BufRead
{
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
                                func: & |mut read| {
                                    let result: Result<_,_> = parse_input(&mut read);
                                    let input = result.map_err(|e| format!("{}", e))?;
                                    let result: Result<_, _> = $year :: $day :: $task (input);
                                    result.map(|x| format!("{}", x))
                                        .map_err(|e| format!("{}", e))
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
        day10 {task1;task2;}
        day11 {task1;task2;}
        day12 {task1;task2;}
        day13 {task1;task2;}
        day14 {task1;task2;}
        day15 {task1;task2;}
        day16 {task1;task2;}
        day17 {task1;task2;}
        day18 {task1;task2;}
        day19 {task1;task2;}
        day20 {task1;task2;}
        day21 {task1;task2;}
        day22 {task1;task2;}
        day23 {task1;task2;}
        day24 {task1;task2;}
        day25 {task1;}
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

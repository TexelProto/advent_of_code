use std::{io::BufRead, fmt::Debug};

pub mod input;
pub mod pathfinding;
pub mod iter_ext;
pub mod debug;


#[derive(Debug)]
pub struct Year {
    pub name: &'static str,
    pub days: &'static [Day],
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

#[derive(Debug)]
pub struct Day {
    pub name: &'static str,
    pub tasks: &'static [Task],
}
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

type TaskFn = dyn Sync + Fn(&mut dyn BufRead) -> Result<String, String>;
pub struct Task {
    pub full_name: &'static str,
    pub name: &'static str,
    pub func: &'static TaskFn,
}
impl Task {
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

#[macro_export]
macro_rules! oneline_dbg {
    () => {
        eprintln!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(oneline_dbg!($val)),+,)
    };
}

#[macro_export]
macro_rules! decl_year {
    (
        $($day:ident {
            $(
                $(#[$attr:ident])*
                $task:ident;
            )*
        })*
    ) => {
        $( pub mod $day; )*
        pub static YEAR: $crate::Year = $crate::Year {
            name: module_path!(),
            days: &[
                $($crate::Day {
                    name: stringify!($day),
                    tasks: &[
                        $($crate::Task {
                            full_name: stringify!($year::$day::$task),
                            name: stringify!($task),
                            func: & |mut read| {
                                fn parse_input<'a, T, R>(read: &'a mut R) -> Result<T, T::Error> 
                                    where T: $crate::input::Input<'a>, R: 'a + std::io::BufRead
                                {
                                    T::parse(read)
                                }

                                let result: Result<_,_> = parse_input(&mut read);
                                let input = result.map_err(|e| format!("{}", e))?;
                                let result: Result<_, _> = $day :: $task (input);
                                result.map(|x| format!("{}", x))
                                    .map_err(|e| format!("{}", e))
                            }
                        },)*
                    ]
                },)*
            ]
        };
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
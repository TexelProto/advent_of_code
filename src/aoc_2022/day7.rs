use std::{
    iter::Peekable,
    str::{FromStr, Lines},
};

trait MultiLineParse<'a> {
    fn parse(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Self;
}

#[derive(Debug, PartialEq, Eq)]
enum Command<'a> {
    Cd(ChangeDirectory<'a>),
    Ls(List<'a>),
}

impl<'a> MultiLineParse<'a> for Command<'a> {
    fn parse(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Self {
        let line = lines.next().unwrap();
        let mut parts = line.split(' ');
        assert_eq!(parts.next().unwrap(), "$");

        match parts.next().unwrap() {
            "cd" => {
                let path = parts.next().unwrap();
                if path == ".." {
                    Command::Cd(ChangeDirectory::Backtrack)
                } else {
                    Command::Cd(ChangeDirectory::Enter(path))
                }
            }
            "ls" => Command::Ls(List::parse(lines)),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ChangeDirectory<'a> {
    Backtrack,
    Enter(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
struct List<'a> {
    results: Vec<ListEntry<'a>>,
}

impl<'a> MultiLineParse<'a> for List<'a> {
    fn parse(lines: &mut Peekable<impl Iterator<Item = &'a str>>) -> Self {
        let mut entries = Vec::new();
        loop {
            if lines.peek() == None {
                break;
            }
            if let Some(line) = lines.peek() {
                if &line[0..1] == "$" {
                    break;
                }
            }

            let line = lines.next().unwrap();
            let mut parts = line.split(' ');
            let first = parts.next().unwrap();
            let name = parts.next().unwrap();

            if first == "dir" {
                entries.push(ListEntry::Directory { name })
            } else {
                let size = usize::from_str(first).unwrap();
                entries.push(ListEntry::File { size: size, name })
            }
        }
        Self { results: entries }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ListEntry<'a> {
    Directory { name: &'a str },
    File { size: usize, name: &'a str },
}

#[derive(Debug, PartialEq, Eq)]
struct Directory<'a> {
    name: &'a str,
    directories: Vec<Directory<'a>>,
    files: Vec<File<'a>>,
}

impl Directory<'_> {
    fn get_total_size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum::<usize>()
            + self
                .directories
                .iter()
                .map(Directory::get_total_size)
                .sum::<usize>()
    }
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Directory {
            name,
            directories: Vec::new(),
            files: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct File<'a> {
    name: &'a str,
    size: usize,
}

fn parse(input: &str) -> Directory {
    let mut input = input.lines().peekable();
    let cmd = Command::parse(&mut input);
    assert_eq!(cmd, Command::Cd(ChangeDirectory::Enter("/")));
    parse_dir("/", &mut input)
}

fn parse_dir<'a>(name: &'a str, input: &mut Peekable<Lines<'a>>) -> Directory<'a> {
    let mut dir = Directory::new(name);

    while input.peek().is_some() {
        match Command::parse(input) {
            Command::Cd(cd) => match cd {
                ChangeDirectory::Backtrack => {
                    break;
                }
                ChangeDirectory::Enter(name) => dir.directories.push(parse_dir(name, input)),
            },
            Command::Ls(list) => {
                for element in list.results {
                    if let ListEntry::File { size, name } = element {
                        dir.files.push(File { name, size })
                    }
                }
            }
        }
    }

    dir
}

fn traverse<'a>(root: &'a Directory, f: &mut impl FnMut(&Directory)) {
    f(root);
    root.directories.iter().for_each(|d| traverse(d, f));
}

pub fn task1(input: String) {
    let tree = parse(&input);

        let mut total_size = 0;
        traverse(&tree, &mut |dir| {
            let size = dir.get_total_size();
            if size <= 100_000 {
                total_size += size;
            }
        });

        dbg!(total_size);
    }
    pub fn task2(input: String) {
        const TOTAL_SPACE: usize = 70_000_000;
        const REQURIED_SPACE: usize = 30_000_000;

        let tree = parse(&input);
        let occupied_space = tree.get_total_size();
        let must_free = occupied_space + REQURIED_SPACE - TOTAL_SPACE;
        dbg!(must_free);

        let mut smallest_free = usize::MAX;
        traverse(&tree, &mut |dir| {
            let size = dir.get_total_size();
            if size >= must_free && size < smallest_free {
                smallest_free = size;
            }
        });

        dbg!(smallest_free);
    }

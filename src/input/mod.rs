use std::fs::File;
use std::io::{BufRead, BufReader, Read};
pub use self::{chars::*, group::*, lines::*};

pub mod chars;
pub mod group;
pub mod lines;

pub type Reader = BufReader<File>;

pub fn parse_lines<E>(
    reader: &mut Reader,
    mut f: impl FnMut(&str) -> Result<(), E>,
) -> Result<(), E>
where
    E: std::error::Error + From<std::io::Error>,
{
    let mut buf = String::with_capacity(256);
    while reader.read_line(&mut buf)? > 0 {
        let s = buf.trim();
        f(s)?;
        buf.clear();
    }
    Ok(())
}

pub trait Input: Sized {
    type Error: std::error::Error;
    fn parse(read: Reader) -> Result<Self, Self::Error>;
}

impl Input for String {
    type Error = std::io::Error;
    fn parse(mut read: Reader) -> Result<Self, Self::Error> {
        let mut s = String::new();
        read.read_to_string(&mut s)?;
        Ok(s)
    }
}
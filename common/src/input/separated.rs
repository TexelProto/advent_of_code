use std::{str::FromStr, marker::PhantomData, io::BufRead, convert::Infallible};

use super::Input;

pub type CommaSeparated<'a, T> = CharSeparated<'a, T, ','>;

pub struct CharSeparated<'a, T: 'a + FromStr, const C: char> {
    input: Box<dyn 'a + BufRead>,
    buffer: String, 
    cursor: usize,
    _t: PhantomData<T>,
}

impl<'a, T: 'a + FromStr, const C: char> Input<'a> for CharSeparated<'a, T, C> {
    type Error = Infallible;

    fn parse<R: 'a + BufRead>(read: R) -> Result<Self, Self::Error> {
        Ok(Self {
            input: Box::new(read),
            buffer: String::new(),
            cursor: 0,
             _t: PhantomData
        })        
    }
}

impl<'a, T: 'a + FromStr, const C: char> Iterator for CharSeparated<'a, T, C> {
    type Item = Result<T, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.buffer.len() {
            self.buffer.clear();
            self.cursor = 0;
            let n = self.input.read_line(&mut self.buffer).unwrap();
            if n == 0 { return None; }
        }

        let read = &self.buffer[self.cursor..];
        let len = read.chars().take_while(|c| {
            debug_assert!(c.is_ascii(), "Cannot handle non ascii input");
            *c != C
        }).count();

        let start = self.cursor;
        let end = self.cursor + len;
        // advance the cursor PAST the separator
        self.cursor += len + 1;

        Some(T::from_str(&self.buffer[start..end]))
    }
}
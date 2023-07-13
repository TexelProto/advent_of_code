use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::*;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::str::FromStr;
use super::Input;

pub(crate) type Reader = BufReader<File>;

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

pub struct Chunked<T: FromStr, const N: usize, const PADDED: bool> {
    read: Reader,
    string: String,
    _t: PhantomData<T>,
}

impl<T: FromStr, const N: usize, const PADDED: bool> Input for Chunked<T, N, PADDED> {
    type Error = Infallible;

    fn parse(read: Reader) -> Result<Self, Self::Error> {
        Ok(Self {
            read,
            string: String::with_capacity(256),
            _t: PhantomData::default(),
        })
    }
}

impl<T: FromStr, const N: usize, const PADDED: bool> Iterator for Chunked<T, N, PADDED> {
    type Item = Result<[T; N], T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut array: [MaybeUninit<T>; N] = std::array::from_fn(|_| MaybeUninit::uninit());
        for i in 0..N {
            let read = self.read.read_line(&mut self.string).unwrap();
            if read == 0 {
                return None;
            }
            let res = T::from_str(self.string.trim());
            self.string.clear();
            let t = match res {
                Ok(t) => t,
                Err(e) => return Some(Err(e)),
            };
            array[i].write(t);
        }

        if PADDED {
            let _ = self.read.read_line(&mut self.string);
            self.string.clear();
        }

        Some(Ok(array.map(|x| unsafe { MaybeUninit::assume_init(x) })))
    }
}

pub struct Grouped<T: FromStr> {
    read: Reader,
    string: String,
    _t: PhantomData<T>,
}

impl<T: FromStr> Input for Grouped<T> {
    type Error = Infallible;

    fn parse(read: Reader) -> Result<Self, Self::Error> {
        Ok(Self {
            read,
            string: String::with_capacity(256),
            _t: PhantomData::default(),
        })
    }
}

impl<T: FromStr> Iterator for Grouped<T> {
    type Item = Result<Vec<T>, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vec = Vec::new();
        loop {
            let read = self.read.read_line(&mut self.string).unwrap();
            let trimmed = self.string.trim();
            if read == 0 || trimmed.len() == 0 {
                break;
            }

            let res = T::from_str(trimmed);
            self.string.clear();
            let t = match res {
                Ok(t) => t,
                Err(e) => return Some(Err(e)),
            };
            vec.push(t);
        }

        if vec.is_empty() {
            None
        } else {
            Some(Ok(vec))
        }
    }
}

use std::convert::Infallible;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::iter::*;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::str::FromStr;

pub(crate) type Reader = BufReader<File>;

pub fn parse_lines<E>(reader: &mut Reader, mut f: impl FnMut(&str) -> Result<(), E>) -> Result<(), E> 
    where E: std::error::Error + From<std::io::Error>{
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

/// Adapter iterator reading from an underlying stream converting each line individually.
/// [`Iterator::next()`] may yield a [`Result::Err`] after which further iteration may become unstable.
/// (Though this will never lead to UB)
pub struct Linewise<T: FromStr> {
    read: Reader,
    string: String,
    _t: PhantomData<T>,
}

impl<T: FromStr> Input for Linewise<T> {
    type Error = Infallible;

    fn parse(read: Reader) -> Result<Self, Self::Error> {
        Ok(Self { read, string: String::with_capacity(256), _t: PhantomData::default() })
    }
}

impl<T: FromStr> Iterator for Linewise<T> {
    type Item = Result<T, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        let read = self.read.read_line(&mut self.string).unwrap();
        if read == 0 {
            return None;
        }
        let t = T::from_str(self.string.trim());
        self.string.clear();
        return Some(t);
    }
}

pub struct Chunked<T: FromStr, const N: usize, const PADDED: bool> {
    read: Reader,
    string: String,
    _t: PhantomData<T>,
}

impl<T: FromStr, const N: usize, const PADDED: bool> Input for Chunked<T, N, PADDED> {
    type Error = Infallible;

    fn parse(read: Reader) -> Result<Self, Self::Error> {
        Ok(Self { read, string: String::with_capacity(256), _t: PhantomData::default() })
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

pub struct GroupedMap<'a, T: FromStr, U, F: FnMut(Vec<T>) -> U>(&'a mut Grouped<T>, F);
impl<'a, T: FromStr, U, F: FnMut(Vec<T>) -> U> Iterator for GroupedMap<'a, T, U, F> {
    type Item = Result<U, T::Err>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(res) => match res {
                Ok(res) => Some(Ok((self.1)(res))),
                Err(err) => Some(Err(err))
            }
        }
    }
}

impl<T: FromStr> Grouped<T> {
    pub fn try_map<U, F: FnMut(Vec<T>) -> U>(&mut self, f: F) -> GroupedMap<T, U, F> {
        GroupedMap(self, f)
    }
}

impl<T: FromStr> Input for Grouped<T> {
    type Error = Infallible;

    fn parse(read: Reader) -> Result<Self, Self::Error> {
        Ok(Self { read, string: String::with_capacity(256), _t: PhantomData::default() })
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


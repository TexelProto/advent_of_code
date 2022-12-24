use std::{str::FromStr, marker::PhantomData, convert::Infallible, mem::MaybeUninit};
use super::*;


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

pub struct GroupedMap<'a, T: FromStr, U, F: FnMut(Vec<T>) -> U>(&'a mut Grouped<T>, F);
impl<'a, T: FromStr, U, F: FnMut(Vec<T>) -> U> Iterator for GroupedMap<'a, T, U, F> {
    type Item = Result<U, T::Err>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(res) => match res {
                Ok(res) => Some(Ok((self.1)(res))),
                Err(err) => Some(Err(err)),
            },
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

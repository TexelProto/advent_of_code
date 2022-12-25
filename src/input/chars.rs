use std::{convert::Infallible, marker::PhantomData, str::FromStr};

pub trait FromChar: Sized {
    type Err: std::error::Error;
    fn from_char(c: char) -> Result<Self, Self::Err>;
}

pub struct Charwise<T: FromChar> {
    str: std::vec::IntoIter<u8>,
    _t: PhantomData<T>,
}

impl<T: FromChar> FromStr for Charwise<T> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            str: s.to_owned().into_bytes().into_iter(),
            _t: PhantomData::default(),
        })
    }
}

impl<T: FromChar> Iterator for Charwise<T> {
    type Item = Result<T, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        self.str.next().map(|b| T::from_char(b as char))
    }
}

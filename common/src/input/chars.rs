use std::{convert::Infallible, marker::PhantomData, str::FromStr};

pub trait FromChar: Sized {
    type Err: std::error::Error;
    fn from_char(c: char) -> Result<Self, Self::Err>;
}

#[derive(thiserror::Error, Debug)]
#[error("Character was not a digit")]
pub struct NonDigitChar;

macro_rules! impl_from_char_int {
    ($($ty:ty),*) => {
        $(
        impl FromChar for $ty {
            type Err = NonDigitChar;
            fn from_char(c: char) -> Result<Self, Self::Err> {
                if c.is_ascii_digit() == false {
                    return Err(NonDigitChar);
                }

                let i = c as u8 - b'0';
                Ok(i as $ty)
            }
        }
        )*
    };
}

impl_from_char_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

impl FromChar for char {
    type Err = Infallible;

    fn from_char(c: char) -> Result<Self, Self::Err> {
        Ok(c)
    }
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
            _t: PhantomData,
        })
    }
}

impl<T: FromChar> Iterator for Charwise<T> {
    type Item = Result<T, T::Err>;

    fn next(&mut self) -> Option<Self::Item> {
        self.str.next().map(|b| T::from_char(b as char))
    }
}

use std::{fmt::Display, marker::PhantomData, num::ParseIntError, str::FromStr, borrow::Cow};

pub use pattern_parse_macros::parse_fn;

pub trait PatternParse: Sized {
    type Error: std::error::Error;
    fn parse(input: &str) -> Result<(Self, usize), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Literal(Cow<'static, str>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parse<T: FromStr>(PhantomData<T>);

pub fn literal(s: impl Into<Cow<'static, str>>) -> Literal {
    Literal(s.into())
}
pub fn parse<T: FromStr>() -> Parse<T> {
    Parse(PhantomData::<T>::default())
}

macro_rules! impl_uint_parse {
    ($($type:ty), *) => {
        $(
            impl PatternParse for $type {
                type Error = ParseIntError;
                fn parse(input: &str) -> Result<(Self, usize), Self::Error> {
                    let len = input.chars().take_while(|c| c.is_numeric()).count();
                    let value = Self::from_str(&input[..len])?;
                    Ok((value, len))
                }
            }
        )*
    };
}

macro_rules! impl_int_parse {
    ($($type:ty), *) => {
        $(
            impl PatternParse for $type {
                type Error = ParseIntError;
                fn parse(input: &str) -> Result<(Self, usize), Self::Error> {
                    let len = input.chars().enumerate().take_while(|(i,c)| {
                        if *i == 0 && (*c == '+' || *c == '-') {
                            true
                        } else {
                            c.is_numeric()
                        }
                    }).count();
                    let value = Self::from_str(&input[..len])?;
                    Ok((value, len))
                }
            }
        )*
    };
}

impl_uint_parse!(u8, u16, u32, u64, u128, usize);
impl_int_parse!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_float_parse {
    ($($type:ty), *) => {
        $(
            impl PatternParse for $type {
                type Error = <$type as FromStr>::Err;
                fn parse(input: &str) -> Result<(Self, usize), Self::Error> {
                    let len = input.chars().take_while(|c| c.is_numeric()).count();
                    let value = Self::from_str(&input[..len])?;
                    Ok((value, len))
                }
            }
        )*
    };
}

impl_float_parse!(f32, f64);

#[derive(Debug)]
pub struct ParseError {
    pub error: Box<dyn 'static + std::error::Error>,
    pub position: usize,
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.error)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing error at position {}: {}", self.position, self.error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralMismatch {
    pub expected: Cow<'static, str>,
    pub got: String,
}

impl Display for LiteralMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mismatched literal expected:\"{}\" got:\"{}\"",
            self.expected, self.got
        )
    }
}

impl std::error::Error for LiteralMismatch {}

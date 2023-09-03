use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum PointParseError {
    #[error("missing separator")]
    MissingSeparator,
    #[error("failed to parse component: {0}")]
    ParseError(#[source] ParseIntError),
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(PointParseError::MissingSeparator)?;
        let x = u32::from_str(x).map_err(|e| PointParseError::ParseError(e))?;
        let y = u32::from_str(y).map_err(|e| PointParseError::ParseError(e))?;
        Ok(Point { x, y })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub const UP: Self = Self { x: 0, y: -1 };
    pub const DOWN: Self = Self { x: 0, y: 1 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };

    pub const CARDINALS: [Self; 4] = [Self::UP, Self::DOWN, Self::LEFT, Self::RIGHT];

    pub const UP_RIGHT: Self = Self { x: 1, y: -1 };
    pub const UP_LEFT: Self = Self { x: -1, y: -1 };
    pub const DOWN_RIGHT: Self = Self { x: 1, y: 1 };
    pub const DOWN_LEFT: Self = Self { x: -1, y: 1 };

    pub const DIAGONALS: [Self; 4] = [
        Self::UP_RIGHT,
        Self::UP_LEFT,
        Self::DOWN_RIGHT,
        Self::DOWN_LEFT,
    ];

    pub const ALL: [Self; 8] = [
        Self::UP,
        Self::DOWN,
        Self::LEFT,
        Self::RIGHT,
        Self::UP_RIGHT,
        Self::UP_LEFT,
        Self::DOWN_RIGHT,
        Self::DOWN_LEFT,
    ];
}

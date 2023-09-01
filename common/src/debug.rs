use std::fmt::{Binary, Debug};

pub struct BinDebug<'a>(pub &'a dyn Binary);

impl Debug for BinDebug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

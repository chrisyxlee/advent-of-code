use std::fmt;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: std::fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

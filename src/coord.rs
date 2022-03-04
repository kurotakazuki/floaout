use std::ops::Sub;

pub type BubFnsCoord = Coord<f64>;

/// Coordinates
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<(T, T, T)> for Coord<T> {
    fn from(coord: (T, T, T)) -> Self {
        Self {
            x: coord.0,
            y: coord.1,
            z: coord.2,
        }
    }
}

impl<T> From<Coord<T>> for (T, T, T) {
    fn from(coord: Coord<T>) -> Self {
        (coord.x, coord.y, coord.z)
    }
}

impl<T> From<Coord<T>> for [T; 3] {
    fn from(coord: Coord<T>) -> Self {
        [coord.x, coord.y, coord.z]
    }
}

impl<T> Sub for Coord<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

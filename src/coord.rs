/// Coordinates
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<(f64, f64, f64)> for Coord {
    fn from(coord: (f64, f64, f64)) -> Self {
        Self {
            x: coord.0,
            y: coord.1,
            z: coord.2,
        }
    }
}

impl From<Coord> for (f64, f64, f64) {
    fn from(coord: Coord) -> Self {
        (coord.x, coord.y, coord.z)
    }
}

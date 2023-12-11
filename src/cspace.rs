pub mod vec2;
pub use vec2::Vec2;

pub mod plane;
pub use plane::Plane;

pub trait Coord2 {
    fn from_tuple(t: (i32, i32)) -> Self;
    fn to_tuple(self) -> (i32, i32);
}

impl Coord2 for (i32, i32) {
    fn from_tuple(t: (i32, i32)) -> Self {
        return t;
    }

    fn to_tuple(self) -> (i32, i32) {
        return self;
    }
}

impl Coord2 for [i32; 2] {
    fn from_tuple(t: (i32, i32)) -> Self {
        return [t.0, t.1];
    }

    fn to_tuple(self) -> (i32, i32) {
        return (self[0], self[1]);
    }
}
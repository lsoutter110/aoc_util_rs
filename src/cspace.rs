pub mod vec2;
pub use vec2::Vec2;

pub mod plane;
pub use plane::Plane;

pub trait Coord2 {
    fn from_tuple(t: (i32, i32)) -> Self;
    fn to_tuple(self) -> (i32, i32);
}
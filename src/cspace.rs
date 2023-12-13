pub mod vec2;
pub use vec2::Vec2;

pub mod plane;
pub use plane::Plane;

pub trait Coord2<N> {
    fn from_tuple(t: (N, N)) -> Self;
    fn to_tuple(self) -> (N, N);
}

impl<N> Coord2<N> for (N, N) {
    fn from_tuple(t: (N, N)) -> Self {
        return t;
    }

    fn to_tuple(self) -> (N, N) {
        return self;
    }
}

impl<N> Coord2<N> for [N; 2] {
    fn from_tuple(t: (N, N)) -> Self {
        return t.into();
    }

    fn to_tuple(self) -> (N, N) {
        return self.into();
    }
}
use super::Coord2;
use std::ops::{Add, Sub};

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Vec2<N> {
    pub x: N,
    pub y: N,
}

impl<N> Vec2<N> where
    N: Add<N, Output = N> + Sub<N, Output = N> + From<u8>
{
    pub fn new(x: N, y: N) -> Vec2<N> {
        return Vec2 { x, y };
    }

    pub fn up(self) -> Vec2<N> {
        return Vec2 { x: self.x, y: self.y - N::from(1) };
    }

    pub fn down(self) -> Vec2<N> {
        return Vec2 { x: self.x, y: self.y + N::from(1) };
    }

    pub fn left(self) -> Vec2<N> {
        return Vec2 { x: self.x - N::from(1), y: self.y };
    }

    pub fn right(self) -> Vec2<N> {
        return Vec2 { x: self.x + N::from(1), y: self.y };
    }
}

impl<N> Coord2<N> for Vec2<N> {
    fn from_tuple(t: (N, N)) -> Vec2<N> {
        return Vec2 { x: t.0, y: t.1 };
    }

    fn to_tuple(self) -> (N, N) {
        return (self.x, self.y);
    }
}

impl<A,N> std::ops::Add<A> for Vec2<N> where
    A: Coord2<N>,
    N: Add<N, Output = N>,
{
    type Output = Vec2<N>;
    fn add(self, rhs: A) -> Self::Output {
        let rhs = rhs.to_tuple();
        return Vec2 { x: self.x + rhs.0, y: self.y + rhs.1 };
    }
}

impl<A,N> std::ops::Sub<A> for Vec2<N> where
    A: Coord2<N>,
    N: Sub<N, Output = N>,
{
    type Output = Vec2<N>;
    fn sub(self, rhs: A) -> Self::Output {
        let rhs = rhs.to_tuple();
        return Vec2 { x: self.x - rhs.0, y: self.y - rhs.1 };
    }
}
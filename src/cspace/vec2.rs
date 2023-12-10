use super::Coord2;

#[derive(Copy,Clone,Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Vec2 {
        return Vec2 { x, y };
    }

    pub fn up(&self) -> Vec2 {
        return Vec2 { x: self.x, y: self.y - 1 };
    }

    pub fn down(&self) -> Vec2 {
        return Vec2 { x: self.x, y: self.y + 1 };
    }

    pub fn left(&self) -> Vec2 {
        return Vec2 { x: self.x - 1, y: self.y };
    }

    pub fn right(&self) -> Vec2 {
        return Vec2 { x: self.x + 1, y: self.y };
    }
}

impl Coord2 for Vec2 {
    fn from_tuple(t: (i32, i32)) -> Vec2 {
        return Vec2 { x: t.0, y: t.1 };
    }

    fn to_tuple(self) -> (i32, i32) {
        return (self.x, self.y);
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        return Vec2 { x: self.x + rhs.x, y: self.y + rhs.y };
    }
}

impl std::ops::Add<(i32, i32)> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        return Vec2 { x: self.x + rhs.0, y: self.y + rhs.1 };
    }
}
use super::Coord2;

pub struct Plane<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Plane<T> {
    fn new() -> Plane<T> {
        return Plane { data: Vec::new(), width: 0, height: 0 };
    }

    fn filled(width: usize, height: usize, item: T) -> Plane<T>
    where T: Clone {
        return Plane { data: vec![vec![item; width]; height], width, height };
    }

    fn push_row(&mut self, row: Vec<T>) {
        if self.height == 0 {
            self.width = row.len();
        }
        assert!(row.len() == self.width, "Row width to push did not match plane width!");
        self.height += 1;
        self.data.push(row);
    }

    fn push_col(&mut self, col: Vec<T>) {
        if self.width == 0 {
            self.height = col.len();
        }
        assert!(col.len() == self.height, "Col width to push did not match plane height!");
        self.width += 1;
        let mut col = col.into_iter();
        for row in &mut self.data {
            row.push(col.next().unwrap());
        }
    }

    fn get<I>(&self, index: I) -> Option<&T>
    where I: Coord2 {
        let t = index.to_tuple();
        return self.data.get(t.1 as usize)?.get(t.0 as usize);
    }

    fn get_mut<I>(&mut self, index: I) -> Option<&mut T>
    where I: Coord2 {
        let t = index.to_tuple();
        return self.data.get_mut(t.1 as usize)?.get_mut(t.0 as usize);
    }
}

use std::ops::{Index, IndexMut};

impl<T,I> Index<I> for Plane<T> where I: Coord2 {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        let t = index.to_tuple();
        return &self.data[t.1 as usize][t.0 as usize];
    }
}

impl<T,I> IndexMut<I> for Plane<T> where I: Coord2 {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let t = index.to_tuple();
        return &mut self.data[t.1 as usize][t.0 as usize];
    }
}


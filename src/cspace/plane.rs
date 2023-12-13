use super::Coord2;

pub struct Plane<T> {
    data: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T> Plane<T> {
    pub fn new() -> Plane<T> {
        return Plane { data: Vec::new(), width: 0, height: 0 };
    }

    pub fn filled(width: usize, height: usize, item: T) -> Plane<T>
    where T: Clone {
        return Plane { data: vec![vec![item; width]; height], width, height };
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        if self.height == 0 {
            self.width = row.len();
        }
        assert!(row.len() == self.width as usize, "Row width to push did not match plane width!");
        self.height += 1;
        self.data.push(row);
    }

    pub fn insert_row(&mut self, index: usize, row: Vec<T>) {
        if self.height == 0 {
            self.width = row.len();
        }
        assert!(row.len() == self.width as usize, "Row width to push did not match plane width!");
        assert!(index <= self.height, "Index outside of plane!");
        self.height += 1;
        self.data.insert(index as usize, row);
    }

    pub fn push_col(&mut self, col: Vec<T>) {
        if self.width == 0 {
            self.height = col.len();
        }
        assert!(col.len() == self.height as usize, "Col width to push did not match plane height!");
        self.width += 1;
        let mut col = col.into_iter();
        for row in &mut self.data {
            row.push(col.next().unwrap());
        }
    }

    pub fn insert_col(&mut self, index: usize, col: Vec<T>) {
        if self.width == 0 {
            self.height = col.len();
        }
        assert!(col.len() == self.height as usize, "Col width to push did not match plane height!");
        assert!(index <= self.width, "Index outside of plane!");
        self.width += 1;
        let mut col = col.into_iter();
        for row in &mut self.data {
            row.insert(index, col.next().unwrap());
        }
    }

    pub fn get<I>(&self, index: I) -> Option<&T>
    where I: Coord2<usize> {
        let t = index.to_tuple();
        return self.data.get(t.1)?.get(t.0);
    }

    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut T>
    where I: Coord2<usize> {
        let t = index.to_tuple();
        return self.data.get_mut(t.1)?.get_mut(t.0);
    }
}

use std::ops::{Index, IndexMut};

impl<T,I> Index<I> for Plane<T> where I: Coord2<usize> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        let t = index.to_tuple();
        return &self.data[t.1 as usize][t.0 as usize];
    }
}

impl<T,I> IndexMut<I> for Plane<T> where I: Coord2<usize> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let t = index.to_tuple();
        return &mut self.data[t.1 as usize][t.0 as usize];
    }
}


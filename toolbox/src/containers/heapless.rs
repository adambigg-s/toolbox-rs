use core::slice;
use std::ops::Index;
use std::ops::IndexMut;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* HEAPLESS VEC */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Vec<T, const N: usize> {
    pub data: [Option<T>; N],
    pub length: usize,
}

impl<T, const N: usize> Vec<T, N>
where
    T: Copy,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.data.fill(None);
        self.length = 0;
    }

    pub fn push(&mut self, value: T) -> Option<()> {
        if !self.len() < N {
            return None;
        }

        self.data[self.len()] = Some(value);
        self.length += 1;
        Some(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.length -= 1;
        let value = self.data[self.len()];
        self.data[self.len()] = None;
        value
    }

    pub fn iter<'d>(&'d self) -> Iter<'d, T, N> {
        Iter::new(self)
    }

    pub fn iter_mut<'d>(&'d mut self) -> IterMut<'d, T, N> {
        IterMut::new(self)
    }
}

impl<T, const N: usize> Default for Vec<T, N>
where
    T: Copy,
{
    fn default() -> Self {
        Self { data: [None; N], length: 0 }
    }
}

impl<T, const N: usize> Index<usize> for Vec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.data[index].as_ref().unwrap()
    }
}

impl<T, const N: usize> IndexMut<usize> for Vec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data[index].as_mut().unwrap()
    }
}

impl<T, const N: usize> FromIterator<T> for Vec<T, N>
where
    T: Copy,
{
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        let mut out = Self::new();
        iter.into_iter().for_each(|item| {
            out.push(item);
        });
        out
    }
}

pub struct Iter<'d, T, const N: usize> {
    vec: &'d Vec<T, N>,
    index: usize,
}

impl<'d, T, const N: usize> Iter<'d, T, N> {
    pub fn new(vec: &'d Vec<T, N>) -> Self {
        Self { vec, index: 0 }
    }
}

impl<'d, T, const N: usize> IntoIterator for &'d Vec<T, N>
where
    T: Copy,
{
    type Item = T;

    type IntoIter = Iter<'d, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl<'d, T, const N: usize> Iterator for Iter<'d, T, N>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.len() {
            return None;
        }

        let value = self.vec.data[self.index];
        self.index += 1;
        value
    }
}

pub struct IterMut<'d, T, const N: usize> {
    data: slice::IterMut<'d, Option<T>>,
}

impl<'d, T, const N: usize> IterMut<'d, T, N>
where
    T: Copy,
{
    pub fn new(vec: &'d mut Vec<T, N>) -> Self {
        Self { data: vec.data[..].iter_mut() }
    }
}

impl<'d, T, const N: usize> IntoIterator for &'d mut Vec<T, N>
where
    T: Copy,
{
    type Item = &'d mut T;

    type IntoIter = IterMut<'d, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

impl<'d, T, const N: usize> Iterator for IterMut<'d, T, N>
where
    T: Copy,
{
    type Item = &'d mut T;

    fn next(&mut self) -> Option<Self::Item> {
        for slot in self.data.by_ref() {
            if let Some(value) = slot.as_mut() {
                return Some(value);
            }
        }

        None
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* TEST */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_iter() {
        let mut vec = Vec::<i32, 20>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let mut iter = vec.into_iter();
        assert!(iter.next().unwrap() == 1);
        assert!(iter.next().unwrap() == 2);
        assert!(iter.next().unwrap() == 3);
        assert!(iter.next().is_none());
    }

    #[test]
    fn vec_iter_mut() {
        let mut vec = Vec::<i32, 5>::new();
        vec.push(1);
        vec.push(1);
        for value in vec.iter_mut() {
            *value += 1;
        }
        let mut iter = vec.into_iter();
        assert!(iter.next().unwrap() == 2);
        assert!(iter.next().unwrap() == 2);
        assert!(iter.next().is_none());
    }

    #[test]
    fn vec_index() {
        let mut vec = Vec::<i32, 5>::new();
        vec.push(10);
        assert!(vec[0] == 10);
    }

    #[test]
    fn vec_index_mut() {
        let mut vec = Vec::<i32, 5>::new();
        vec.push(10);
        vec[0] = 100;
        assert!(vec[0] == 100);
    }
}

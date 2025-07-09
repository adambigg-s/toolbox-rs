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
}

impl<T, const N: usize> Default for Vec<T, N>
where
    T: Copy,
{
    fn default() -> Self {
        Self { data: [None; N], length: 0 }
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

pub struct Iter<'d, T, const N: usize> {
    vec: &'d Vec<T, N>,
    index: usize,
}

impl<'d, T, const N: usize> Iter<'d, T, N> {
    pub fn new(vec: &'d Vec<T, N>) -> Self {
        Self { vec, index: 0 }
    }
}

impl<'d, T, const N: usize> Iterator for Iter<'d, T, N>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.index < self.vec.len() {
            return None;
        }

        let value = self.vec.data[self.index];
        self.index += 1;
        value
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
}

pub struct Vec<T, const N: usize> {
    data: [Option<T>; N],
    length: usize,
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
        self.data[self.len()]
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

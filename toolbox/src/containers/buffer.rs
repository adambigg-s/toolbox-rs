/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* BUFFER 2D */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Buffer2<T> {
    pub data: Vec<T>,
    pub clear_value: T,
    pub width: usize,
    pub height: usize,
}

impl<T> Buffer2<T>
where
    T: Clone + Copy,
{
    pub fn new(width: usize, height: usize, clear_value: T) -> Self {
        Self {
            data: vec![clear_value; width * height],
            clear_value,
            width,
            height,
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(self.clear_value);
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if !self.inbounds(x, y) {
            return None;
        }

        let index = self.index(x, y);
        Some(self.data[index])
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> T {
        let index = self.index(x, y);
        self.data[index]
    }

    pub fn set(&mut self, x: usize, y: usize, data: T) -> Option<()> {
        if !self.inbounds(x, y) {
            return None;
        }

        let index = self.index(x, y);
        self.data[index] = data;
        Some(())
    }

    #[inline]
    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    #[inline]
    fn index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* BUFFER 3D */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Buffer3<T> {
    pub data: Vec<T>,
    pub clear_value: T,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

impl<T> Buffer3<T>
where
    T: Clone + Copy,
{
    pub fn new(width: usize, height: usize, depth: usize, clear_value: T) -> Self {
        Self {
            data: vec![clear_value; width * height * depth],
            clear_value,
            width,
            height,
            depth,
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(self.clear_value);
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<T> {
        if !self.inbounds(x, y, z) {
            return None;
        }

        let index = self.index(x, y, z);
        Some(self.data[index])
    }

    pub fn get_unchecked(&self, x: usize, y: usize, z: usize) -> T {
        let index = self.index(x, y, z);
        self.data[index]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, data: T) -> Option<()> {
        if !self.inbounds(x, y, z) {
            return None;
        }

        let index = self.index(x, y, z);
        self.data[index] = data;
        Some(())
    }

    #[inline]
    pub fn inbounds(&self, x: usize, y: usize, z: usize) -> bool {
        x < self.width && y < self.height && z < self.depth
    }

    #[inline]
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        self.width * self.height * z + self.width * y + x
    }
}

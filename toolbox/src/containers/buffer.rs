use crate::math::traits::Scalar;
use crate::math::vector::Vector2;
use crate::math::vector::Vector3;
use crate::vector;

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

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, data: T) -> Option<()> {
        if !self.inbounds(x, y) {
            return None;
        }

        let index = self.index(x, y);
        self.data[index] = data;
        Some(())
    }

    #[inline(always)]
    pub fn set_unchecked(&mut self, x: usize, y: usize, data: T) {
        let index = self.index(x, y);
        self.data[index] = data;
    }

    #[inline(always)]
    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    #[inline(always)]
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

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, z: usize, data: T) -> Option<()> {
        if !self.inbounds(x, y, z) {
            return None;
        }

        let index = self.index(x, y, z);
        self.data[index] = data;
        Some(())
    }

    #[inline(always)]
    pub fn set_unchecked(&mut self, x: usize, y: usize, z: usize, data: T) {
        let index = self.index(x, y, z);
        self.data[index] = data;
    }

    #[inline(always)]
    pub fn inbounds(&self, x: usize, y: usize, z: usize) -> bool {
        x < self.width && y < self.height && z < self.depth
    }

    #[inline(always)]
    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        self.width * self.height * z + self.width * y + x
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* RENDER BITMAP */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Bitmap<T, D> {
    pub main: Buffer2<T>,
    pub depth: Buffer2<f32>,
    pub dim: Vector2<usize>,

    pub util: Vec<D>,
}

impl<T, D> Bitmap<T, D>
where
    T: Scalar,
{
    pub fn new(width: usize, height: usize, main_clear: T, depth_clear: f32) -> Self {
        Self {
            main: Buffer2::new(width, height, main_clear),
            depth: Buffer2::new(width, height, depth_clear),
            dim: vector!(usize; width, height),

            util: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.main.clear();
        self.depth.clear();
    }

    pub fn update_from(&mut self, payload: BitmapPayload<T>) -> Option<()> {
        let [x, y] = payload.position.array();
        if !self.inbounds(x, y) {
            return None;
        }

        self.main.set_unchecked(x, y, payload.data);
        self.depth.set_unchecked(x, y, payload.depth);
        Some(())
    }

    pub fn inbounds(&self, x: usize, y: usize) -> bool {
        self.dim.x > x && self.dim.y > y
    }
}

pub struct BitmapPayload<T> {
    pub position: Vector2<usize>,
    pub data: T,
    pub depth: f32,
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* BITMAP UTILS */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub type Color = Vector3<f32>;

pub type PackedColor = u32;

pub fn pack_color(color: Color) -> PackedColor {
    let [r_pre, g_pre, b_pre] = color.array();
    let r = (r_pre * 255.) as u32;
    let g = (g_pre * 255.) as u32;
    let b = (b_pre * 255.) as u32;

    (0xff << 24) | (r << 16) | (g << 8) | b
}

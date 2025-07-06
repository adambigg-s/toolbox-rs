#[macro_export]
macro_rules! vec {
    ($a:expr, $b:expr) => {
        vec2!($a, $b)
    };
    ($type:ty; $a:expr, $b: expr) => {
        vec2!($type; $a, $b)
    };

    ($a:expr, $b:expr, $c:expr) => {
        vec3!($a, $b, $c)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr) => {
        vec3!($type; $a, $b, $c)
    };

    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        vec4!($a, $b, $c, $d)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr, $d:expr) => {
        vec4!($type; $a, $b, $c, $d)
    };
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR 2 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[macro_export]
macro_rules! vec2 {
    ($a:expr, $b:expr) => {
        Vector2::build($a as f32, $b as f32)
    };
    ($type:ty; $a:expr, $b: expr) => {
        Vector2::build($a as $type, $b as $type)
    };
}

impl<T> Vector2<T> {
    pub fn build(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR 3 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[macro_export]
macro_rules! vec3 {
    ($a:expr, $b:expr, $c:expr) => {
        Vector3::build($a as f32, $b as f32, $c as f32)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr) => {
        Vector3::build($a as $type, $b as $type, $c as $type)
    };
}

impl<T> Vector3<T> {
    pub fn build(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR 4 */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[macro_export]
macro_rules! vec4 {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        Vector4::build($a as f32, $b as f32, $c as f32, $d as f32)
    };
    ($type:ty; $a:expr, $b:expr, $c:expr, $d:expr) => {
        Vector4::build($a as $type, $b as $type, $c as $type, $d as $type)
    };
}

impl<T> Vector4<T> {
    pub fn build(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* VECTOR N*/
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VectorN<T, const N: usize> {
    inner: [T; N],
}

impl<T, const N: usize> VectorN<T, N> {
    pub fn build(inner: [T; N]) -> Self {
        Self { inner }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* TEST */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macros() {
        let vector = vec!(i32; 4, 10, 4.3, -5);
        assert!(vector == Vector4::build(4, 10, 4, -5));
    }
}

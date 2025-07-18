use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* THREAD POINTER */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct RawPtr<T> {
    ptr: *mut T,
}

impl<T> RawPtr<T> {
    pub fn new(data: &mut T) -> Self {
        Self { ptr: data as *mut T }
    }

    pub fn read(&self) -> &T {
        unsafe { &(*self.ptr) }
    }

    pub fn drop_not(&mut self) {
        unsafe { _ = Box::from_raw(self.ptr) }
    }
}

unsafe impl<T> Send for RawPtr<T> {}

unsafe impl<T> Sync for RawPtr<T> {}

impl<T> Copy for RawPtr<T> {}

impl<T> Clone for RawPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Deref for RawPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.read()
    }
}

impl<T> DerefMut for RawPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*self.ptr) }
    }
}

impl<T> From<Box<T>> for RawPtr<T> {
    fn from(value: Box<T>) -> Self {
        Self { ptr: Box::into_raw(value) }
    }
}

impl<T> From<Arc<T>> for RawPtr<T> {
    fn from(value: Arc<T>) -> Self {
        Self { ptr: Arc::into_raw(value) as *mut T }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////
/* TEST */
/////////////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ptr() {
        let mut data = 120;
        let mut mut1 = RawPtr::new(&mut data);
        let mut mut2 = RawPtr::new(&mut data);
        *mut1 += 1;
        *mut2 += 10;
        assert!(data == 131);
    }
}

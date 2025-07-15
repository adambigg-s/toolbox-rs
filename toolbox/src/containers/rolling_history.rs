use std::collections::LinkedList;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct RollingHistory<T> {
    pub log: LinkedList<T>,
    pub len: usize,
}

impl<T> RollingHistory<T> {
    pub fn build(length: usize) -> Self {
        Self { log: LinkedList::new(), len: length }
    }

    pub fn push(&mut self, state: T) -> Option<()> {
        if self.log.len() > self.len {
            self.log.pop_front();
            self.log.push_back(state);
            return None;
        }

        self.log.push_back(state);
        Some(())
    }
}

impl<T> Deref for RollingHistory<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.log
    }
}

impl<T> DerefMut for RollingHistory<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.log
    }
}

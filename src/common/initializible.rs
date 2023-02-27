use core::ops::{Deref, DerefMut};

pub struct Initializible<T> {
    is_initialized: bool,
    value: *mut T,
}

impl<T> Initializible<T> {
    pub const fn new() -> Initializible<T> {
        return Initializible {
            is_initialized: false,
            value: 0 as *mut T,
        };
    }

    pub fn initialize(&mut self, value: *mut T) {
        self.value = value;
        self.is_initialized = true;
    }
}

impl<T> Deref for Initializible<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if !self.is_initialized {
            panic!();
        }

        return unsafe { &*self.value };
    }
}

impl<T> DerefMut for Initializible<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if !self.is_initialized {
            panic!();
        }

        return unsafe { &mut *self.value };
    }
}

#![no_std]

extern crate result;

use result::Result;

pub trait TryClone {
    fn try_clone(&self) -> Result<Self>
    where
        Self: Sized;
}

impl<T: Clone> TryClone for T {
    fn try_clone(&self) -> Result<Self> {
        Ok(self.clone())
    }
}

pub fn real_main(_argc: i32, _argv: *const *const i8) -> i32 {
    1
}

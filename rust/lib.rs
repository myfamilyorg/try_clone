#![no_std]

extern crate macros;
extern crate result;
use macros::Dummy;

#[derive(Dummy)]
struct MyStruct {
    x: i32,
}

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

pub fn real_main(argc: i32, _argv: *const *const i8) -> i32 {
    let x = MyStruct { x: 3 };
    let y = MyStruct { x: argc };
    if x == y {
        x.x
    } else {
        0
    }
}

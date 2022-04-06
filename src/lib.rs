#![no_std]
#![warn(rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn)]

pub mod ptr;
mod strategy;

pub use stuff::Backend;

use crate::strategy::{SafeStrategyAdaptor, SafeStuffingStrategy};

pub struct SafeStuffedPtr<T, S, B = usize>(stuff::StuffedPtr<T, SafeStrategyAdaptor<S>, B>)
where
    S: SafeStuffingStrategy<B>,
    B: Backend<T>,
    B: TryInto<usize>,
    usize: TryInto<B>;

impl<T, S, B> SafeStuffedPtr<T, S, B>
where
    S: SafeStuffingStrategy<B>,
    B: Backend<T>,
    B: TryInto<usize>,
    usize: TryInto<B>,
{
    pub fn new_ptr(ptr: *mut T) -> Self {
        Self(stuff::StuffedPtr::new_ptr(ptr))
    }

    pub fn new_extra(ptr: *mut T) -> Self {
        Self(stuff::StuffedPtr::new_ptr(ptr))
    }

    pub fn get_ptr(&self) -> Option<*const T> {
        self.0.get_ptr().map(|ptr| ptr as *const _)
    }

    pub fn get_mut_ptr(&mut self) -> Option<*mut T> {
        self.0.get_ptr()
    }

    pub fn into_other(self) -> Option<S::Other> {
        self.0.into_other()
    }
}

impl<T, S, B> SafeStuffedPtr<T, S, B>
where
    S: SafeStuffingStrategy<B>,
    B: Backend<T>,
    B: TryInto<usize>,
    usize: TryInto<B>,
    S::Other: Copy,
{
    pub fn copy_other(&self) -> Option<S::Other> {
        self.0.copy_other()
    }
}

#![no_std]
#![warn(rust_2018_idioms)]
#![deny(unsafe_op_in_unsafe_fn)]

mod strategy;

pub use stuff::Backend;

use crate::strategy::{SafeStrategyAdaptor, SafeStuffingStrategy};

pub struct StuffedPtr<T, S, B = usize>(stuff::StuffedPtr<T, SafeStrategyAdaptor<S>, B>)
where
    S: SafeStuffingStrategy<B>,
    B: Backend<T>;

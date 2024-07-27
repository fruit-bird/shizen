#![feature(unboxed_closures)]
#![feature(fn_traits)]

mod delay;
mod gain;

pub use crate::delay::DelayComponent;
pub use crate::gain::GainComponent;

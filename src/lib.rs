#![allow(unused_unsafe)]

#[macro_use]
mod shared;

mod alexandrescu;
mod jeaiii;
mod naive;

pub use alexandrescu::alexandrescu32;
pub use jeaiii::{jeaiii32_better, jeaiii32_digits, jeaiii32_10, jeaiii32_original};
pub use naive::{naive32_exact, naive32_temp};

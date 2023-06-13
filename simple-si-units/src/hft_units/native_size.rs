
//! This module provides geometry SI units, such as angle
//! and inverse of volume.
// FIXME remove
#[warn(unused_imports)]

use core::fmt;
use crate::{commutative_mul, mul_unit};

use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::cmp::PartialEq;



/// TODO
#[derive(PartialEq, Debug, Clone, Copy)]
// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct NativeSize {
    /// TODO
    pub amount: u64
}


impl NativeSize {

    pub fn unit_name() -> &'static str { "native size" }

    pub fn unit_symbol() -> &'static str { "nsz" }

    // TODO find better name
    pub fn from(amount: u64) -> Self {
        NativeSize {
            amount: amount,
        }
    }

}

impl fmt::Display for NativeSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.amount, Self::unit_symbol())
    }
}


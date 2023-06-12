
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

use super::price::Price;
use super::size::Size;


/// TODO
#[derive(PartialEq, Debug, Clone, Copy)]
// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Notional {
    /// TODO
    pub value: u64
}

// Price * Size -> Notional
impl Notional {

    pub fn unit_name() -> &'static str { "notional" }

    pub fn unit_symbol() -> &'static str { "notional" }

}

impl fmt::Display for Notional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.value, Self::unit_symbol())
    }
}

commutative_mul!( Size, Price, Notional);

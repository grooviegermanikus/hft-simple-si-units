
//! This module provides quantity in lots
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
pub struct Lots {
    /// TODO
    pub amount: u64
}


impl Lots {

    pub fn unit_symbol() -> &'static str { "lots" }

    pub fn unit_name() -> &'static str { "number of lots" }

    // TODO find better name
    pub fn from(amount: u64) -> Self {
        Lots {
            amount: amount,
        }
    }

}

impl fmt::Display for Lots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.amount, Self::unit_symbol())
    }
}


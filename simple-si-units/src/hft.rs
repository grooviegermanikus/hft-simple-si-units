//! This module provides geometry SI units, such as angle
//! and inverse of volume.
use core::fmt;
use crate::{mul_div_unit};
use super::UnitStruct;
use super::NumLike;

/// TODO
#[derive(PartialEq, Debug, Clone, Copy)]
// #[derive(UnitStruct, Debug, Clone)]
// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Price {
    /// TODO
    pub value: u64
}


impl Price {

    pub fn unit_name() -> &'static str { "price units" }

    pub fn unit_symbol() -> &'static str { "price units" }

    pub fn from(value: u64) -> Self {
        Price {
            value: value,
        }
    }

}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.value, Self::unit_symbol())
    }
}


/// TODO
#[derive(PartialEq, Debug, Clone, Copy)]
// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Size {
    /// TODO
    pub amount: u64
}

impl Size {

    pub fn unit_name() -> &'static str { "size" }

    pub fn unit_symbol() -> &'static str { "size" }

    pub fn from(amount: u64) -> Self {
        Size {
            amount: amount,
        }
    }

}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.amount, Self::unit_symbol())
    }
}

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


// impl core::ops::Mul<Size> for Price {
//     type Output = Notional;
//     fn mul(self, rhs: Size) -> Self::Output {
//         Notional{value: self.value * rhs.amount}
//     }
// }
//
// impl core::ops::Mul<Price> for Size {
//     type Output = Notional;
//     fn mul(self, rhs: Price) -> Self::Output {
//         Notional{value: self.amount * rhs.value}
//     }
// }

use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::cmp::PartialEq;

fn my_mult(lhs: Price, rhs: Size) -> Notional {
    Notional {
        value: lhs.value * rhs.amount,
    }
}
mul_div_unit!(Price, Size, Notional, my_mult);
// implement deref

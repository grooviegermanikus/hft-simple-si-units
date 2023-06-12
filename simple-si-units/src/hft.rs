//! This module provides geometry SI units, such as angle
//! and inverse of volume.
use core::fmt;
use crate::{commutative_mul, mul_unit};
use super::UnitStruct;
use super::NumLike;

use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::cmp::PartialEq;

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

    // TODO find better name
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


impl core::ops::Mul<Size> for Price {
    type Output = Notional;
    fn mul(self, rhs: Size) -> Self::Output {
        Notional{value: self.value * rhs.amount}
    }
}

commutative_mul!( Size, Price, Notional);
// impl core::ops::Mul<Price> for Size {
//     type Output = Notional;
//     fn mul(self, rhs: Price) -> Self::Output {
//         rhs.mul(self)
//     }
// }



fn my_mul(lhs: Price, rhs: Size) -> Notional {
    Notional {
        value: lhs.value * rhs.amount,
    }
}

fn my_div_by2(lhs: Notional, rhs: Size) -> Price {
    Price {
        value: lhs.value / rhs.amount,
    }
}

fn my_div_by1(lhs: Notional, rhs: Price) -> Size {
    Size {
        amount: lhs.value / rhs.value,
    }
}

// mul_unit!(Price, Size, Notional, my_mul, my_div_by2, my_div_by1);
// implement deref

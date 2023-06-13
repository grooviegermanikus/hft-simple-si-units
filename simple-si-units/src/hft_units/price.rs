use core::fmt;

use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::cmp::PartialEq;
use crate::hft_units::{Notional, NativeSize};


/// TODO
#[derive(PartialEq, Debug, Clone, Copy)]
// #[derive(UnitStruct, Debug, Clone)]
// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Price {
    /// TODO
    pub value: u64
}


impl Price {

    pub fn unit_symbol() -> &'static str { "pc" }

    pub fn unit_name() -> &'static str { "price units" }


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

impl core::ops::Mul<NativeSize> for Price {
        type Output = Notional;
        fn mul(self, rhs: NativeSize) -> Self::Output {
            Notional{value: self.value * rhs.amount}
        }
}

use core::fmt;

use core::ops::Add;
use core::ops::Sub;
use core::ops::Mul;
use core::ops::Div;
use core::cmp::PartialEq;


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct NativeSizePerLot {
    pub native_per_lot: u64,
}

impl NativeSizePerLot {

    pub fn unit_symbol() -> &'static str { "nsz/lot" }

    pub fn unit_name() -> &'static str { "native size per lot" }

    pub fn from_native_per_lots(native_per_lot: u64) -> Self {
        NativeSizePerLot {
            native_per_lot,
        }
    }

}

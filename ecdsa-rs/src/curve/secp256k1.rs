use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::{Num, Zero};

use crate::field::FieldElement;

use super::Point;

pub fn a() -> BigInt {
    BigInt::zero()
}

pub fn b() -> BigInt {
    BigInt::from(7u8)
}

pub fn p() -> BigInt {
    BigInt::from_str_radix(
        "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
        16,
    )
    .expect("failed to construct bigInt from Hex")
}

pub fn n() -> BigInt {
    BigInt::from_str_radix(
        "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .expect("failed to construct bigInt from Hex")
}

pub fn gx() -> BigInt {
    BigInt::from_str_radix(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16,
    )
    .expect("failed to construct bigInt from Hex")
}
pub fn gy() -> BigInt {
    BigInt::from_str_radix(
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        16,
    )
    .expect("failed to construct bigInt from Hex")
}
pub fn g() -> Point {
    Point {
        x: Some(FieldElement {
            value: gx(),
            field_modulus: p(),
        }),
        y: Some(FieldElement {
            value: gy(),
            field_modulus: p(),
        }),
    }
}

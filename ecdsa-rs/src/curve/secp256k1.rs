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
    .expect("Failed to construct BigInt from Hex")
}

pub fn gx() -> BigInt {
    BigInt::from_str_radix(
        "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
        16,
    )
    .expect("Failed to construct BigInt from Hex")
}
pub fn gy() -> BigInt {
    BigInt::from_str_radix(
        "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
        16,
    )
    .expect("Failed to construct BigInt from Hex")
}
pub fn g() -> Point {
    Point {
        x: Some(FieldElement {
            value: Rc::new(gx()),
            field_modulus: Rc::new(p()),
        }),
        y: Some(FieldElement {
            value: Rc::new(gy()),
            field_modulus: Rc::new(p()),
        }),
    }
}

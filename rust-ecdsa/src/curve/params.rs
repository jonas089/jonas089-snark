use crate::curve::Point;
use num_bigint::BigInt;
use num_traits::{Num, One, Pow, Zero};

pub const secp256k1_Gx_hex: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
pub const secp256k1_Gy_hex: &str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";

pub fn two() -> BigInt{
    BigInt::from(2u8)
}

pub fn three() -> BigInt{
    BigInt::from(3u8)
}

pub fn secp256k1_a() -> BigInt{
    BigInt::zero()
}

pub fn secp256k1_b() -> BigInt{
    BigInt::from(7u8)
}

pub fn secp256k1_p() -> BigInt{
    let base: BigInt = BigInt::one() + BigInt::one();
    let two_pow_256: BigInt = base.clone().pow(256u32);
    let two_pow_32: BigInt = base.pow(32u32);
    let nine_seven_seven: BigInt = 977u32.into();
    let result: BigInt = two_pow_256 - two_pow_32 - nine_seven_seven;
    result
}

pub fn secp256k1_gx() -> BigInt{
    BigInt::from_str_radix(secp256k1_Gx_hex, 16).expect("Failed to construct BigInt from Hex")
}

pub fn secp256k1_gy() -> BigInt{
    BigInt::from_str_radix(secp256k1_Gy_hex, 16).expect("Failed to construct BigInt from Hex")
}

pub fn secp256k1_g() -> Point{
    Point{
        x: Some(secp256k1_gx()),
        y: Some(secp256k1_gy())
    }
}
use crate::curve::Point;
use num_bigint::BigInt;
use num_traits::{Num, One, Pow, Zero};

// constants
pub fn two() -> BigInt{
    BigInt::from(2u8)
}

pub fn three() -> BigInt{
    BigInt::from(3u8)
}

// Secp256k1 - Bitcoin curve, not pairing friendly
pub struct SECP_256_K1{}
impl SECP_256_K1{
    pub fn a(&self) -> BigInt{
        BigInt::zero()
    }
    
    pub fn b(&self) -> BigInt{
        BigInt::from(7u8)
    }
    
    pub fn p(&self) -> BigInt{
        BigInt::from_str_radix("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).expect("Failed to construct BigInt from Hex")
    }

    // x-coordinate of the generator point 1G
    pub fn gx(&self) -> BigInt{
        BigInt::from_str_radix("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).expect("Failed to construct BigInt from Hex")
    }
    // y-coordinate of the generator point 1G
    pub fn gy(&self) -> BigInt{
        BigInt::from_str_radix("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).expect("Failed to construct BigInt from Hex")
    }
    // Point form of the generator point 1G
    pub fn g(&self) -> Point{
        Point{
            x: Some(self.gx()),
            y: Some(self.gy())
        }
    }
}

// BLS12-381 - pairing friendly elliptic curve
pub struct BLS_12_381{}
impl BLS_12_381{
    pub fn a(&self) -> BigInt{
        BigInt::zero()
    }
    
    pub fn b(&self) -> BigInt{
        BigInt::from(4u8)
    }

    pub fn p(&self) -> BigInt{
        BigInt::from_str_radix("TBD", 16).expect("Failed to construct BigInt from Hex")
    }

    // x-coordinate of the generator point 1G
    pub fn gx(&self) -> BigInt{
        BigInt::from_str_radix("TBD", 16).expect("Failed to construct BigInt from Hex")
    }
    
    // y-coordinate of the generator point 1G
    pub fn gy(&self) -> BigInt{
        BigInt::from_str_radix("TBD", 16).expect("Failed to construct BigInt from Hex")
    }

    // Point form of the generator point 1G
    pub fn g(&self) -> Point{
        Point{
            x: Some(self.gx()),
            y: Some(self.gy())
        }
    }   
}
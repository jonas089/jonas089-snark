use num_bigint::BigInt;
use crate::math::{modulo, prime_field_inv};
pub struct FQ{
    pub n: BigInt,
    pub field_modulus: BigInt
}

impl FQ{
    /// create a new FQ element from another FQ element
    /// or from a BigInt
    pub fn new(&mut self, val: FqElement, field_modulus: BigInt) -> FQ{
        match val{
            FqElement::FQ(val) => {
                FQ{
                    n: val.n,
                    field_modulus: field_modulus
                }
            },
            FqElement::BigInt(n) => {
                FQ{
                    n: modulo(&n, &field_modulus),
                    field_modulus
                }
            }
        }
    }
    
    pub fn add(&self, other: FqElement) -> FQ{
        match other{
            FqElement::FQ(other) => {
                let out = modulo(&(&self.n + other.n), &self.field_modulus);
                FQ{
                    n: out,
                    field_modulus: self.field_modulus.clone()
                }
            },
            FqElement::BigInt(n) => {
                let out = modulo(&(&self.n + n), &self.field_modulus);
                FQ{
                    n: out,
                    field_modulus: self.field_modulus.clone()
                }
            }
        }
    }

    pub fn mul(&self, other: FqElement) -> FQ{
        match other{
            FqElement::FQ(other) => {
                let out = modulo(&(&self.n * other.n), &self.field_modulus);
                FQ{
                    n: out,
                    field_modulus: self.field_modulus.clone()
                }
            },
            FqElement::BigInt(n) => {
                let out = modulo(&(&self.n * n), &self.field_modulus);
                FQ{
                    n: out,
                    field_modulus: self.field_modulus.clone()
                }
            }
        }
    }

    pub fn div(&self, other: FqElement) -> FQ{
        match other{
            FqElement::FQ(other) => {
                let out = modulo(&(&self.n * &prime_field_inv(other.n, self.field_modulus.clone())), &self.field_modulus);
                FQ{
                    n: out,
                    field_modulus: self.field_modulus.clone()
                }
            },
            FqElement::BigInt(n) => {
                let out = modulo(&(&self.n * &prime_field_inv(n, self.field_modulus.clone())), &self.field_modulus);
                FQ{
                    n: out,
                    field_modulus: self.field_modulus.clone()
                }
            }
        }
    }

    pub fn pow(&self, other: FqElement) -> FQ{
        match other{
            FqElement::FQ(other) => {
                let out = self.n.modpow(&other.n, &self.field_modulus);
                FQ{
                    n: out,
                    field_modulus: self.field_modulus.clone()
                }
            },
            FqElement::BigInt(n) => {
                let out = self.n.modpow(&n, &self.field_modulus);
                FQ{
                    n: out,
                    field_modulus: self.field_modulus.clone()
                }
            }
        }
    }
}

enum FqElement{
    FQ(FQ),
    BigInt(BigInt)
}
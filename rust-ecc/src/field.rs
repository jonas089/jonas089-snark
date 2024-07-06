use crate::math::{modulo, prime_field_inv};
use num_bigint::BigInt;

#[derive(Default)]
pub struct FQ {
    pub n: BigInt,
    pub field_modulus: BigInt,
}

impl FQ {
    /// create a new FQ element from another FQ element
    /// or from a BigInt
    pub fn new(&mut self, val: FqElement, field_modulus: BigInt) -> FQ {
        match val {
            FqElement::FQ(val) => FQ {
                n: val.n,
                field_modulus: field_modulus,
            },
            FqElement::BigInt(n) => FQ {
                n: modulo(&n, &field_modulus),
                field_modulus,
            },
        }
    }

    pub fn add(&self, other: FqElement) -> FQ {
        match other {
            FqElement::FQ(other) => {
                let out = modulo(&(&self.n + other.n), &self.field_modulus);
                FQ {
                    n: out,
                    field_modulus: self.field_modulus.clone(),
                }
            }
            FqElement::BigInt(n) => {
                let out = modulo(&(&self.n + n), &self.field_modulus);
                FQ {
                    n: out,
                    field_modulus: self.field_modulus.clone(),
                }
            }
        }
    }

    pub fn mul(&self, other: FqElement) -> FQ {
        match other {
            FqElement::FQ(other) => {
                let out = modulo(&(&self.n * other.n), &self.field_modulus);
                FQ {
                    n: out,
                    field_modulus: self.field_modulus.clone(),
                }
            }
            FqElement::BigInt(n) => {
                let out = modulo(&(&self.n * n), &self.field_modulus);
                FQ {
                    n: out,
                    field_modulus: self.field_modulus.clone(),
                }
            }
        }
    }

    pub fn div(&self, other: FqElement) -> FQ {
        match other {
            FqElement::FQ(other) => {
                let out = modulo(
                    &(&self.n * &prime_field_inv(other.n, self.field_modulus.clone())),
                    &self.field_modulus,
                );
                FQ {
                    n: out,
                    field_modulus: self.field_modulus.clone(),
                }
            }
            FqElement::BigInt(n) => {
                let out = modulo(
                    &(&self.n * &prime_field_inv(n, self.field_modulus.clone())),
                    &self.field_modulus,
                );
                FQ {
                    n: out,
                    field_modulus: self.field_modulus.clone(),
                }
            }
        }
    }

    pub fn pow(&self, other: FqElement) -> FQ {
        match other {
            FqElement::FQ(other) => {
                let out = self.n.modpow(&other.n, &self.field_modulus);
                FQ {
                    n: out,
                    field_modulus: self.field_modulus.clone(),
                }
            }
            FqElement::BigInt(n) => {
                let out = self.n.modpow(&n, &self.field_modulus);
                FQ {
                    n: out,
                    field_modulus: self.field_modulus.clone(),
                }
            }
        }
    }

    pub fn eq(&self, other: FqElement) -> bool {
        match other {
            FqElement::FQ(other) => self.n == other.n,
            FqElement::BigInt(n) => self.n == n,
        }
    }
}

enum FqElement {
    FQ(FQ),
    BigInt(BigInt),
}

/// Struct for multidimensional elliptic curve points
#[derive(Default)]
pub struct FQP {
    degree: BigInt,
    field_modulus: BigInt,
    coeffs: Vec<FQ>,
    modulus_coeffs: Vec<FqElement>,
}

impl FQP {
    pub fn new(&mut self, coeffs: Vec<FqElement>, modulus_coeffs: Vec<FqElement>) {
        if coeffs.len() != modulus_coeffs.len() {
            panic!("coeffs and modulus_coeffs aren't of the same length");
        };
        // the corresponding fq class is differentiated only by its field modulus,
        // therefore we construct a default instance and assign the field modulus when
        // instantiating an element
        let mut corresponding_fq_class: FQ = FQ::default();
        let mut fq_class_coeffs: Vec<FQ> = Vec::new();
        // convert each coefficient to an FQ element
        for c in coeffs {
            fq_class_coeffs.push(corresponding_fq_class.new(c, self.field_modulus.clone()));
        }
        self.coeffs = fq_class_coeffs;
        self.modulus_coeffs = modulus_coeffs;
        self.degree = BigInt::from(self.modulus_coeffs.len());
    }
}

// References from py_ecc

/*field_properties = {
    "bn128": {
        "field_modulus": 21888242871839275222246405745257275088696311157297823662689037894645226208583,  # noqa: E501
        "fq2_modulus_coeffs": (1, 0),
        "fq12_modulus_coeffs": (82, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0),  # Implied + [1]
    },
}*/

use num_bigint::BigInt;
use num_traits::Zero;
use std::{borrow::Borrow, ops::{Add, Mul, Rem, Sub}, rc::Rc, cell::RefCell};
#[derive(Debug, Clone)]
pub struct FieldElement {
    pub value: BigInt,
    pub field_modulus: Rc<BigInt>,
}

impl Rem<&FieldElement> for FieldElement {
    type Output = FieldElement;
    fn rem(self, other: &FieldElement) -> FieldElement {
        let result: BigInt = &self.value % &other.value;
        if result < BigInt::zero() {
            return FieldElement {
                value: &result + &other.value,
                field_modulus: self.field_modulus.clone(),
            };
        };
        FieldElement {
            value: result,
            field_modulus: self.field_modulus.clone(),
        }
    }
}

impl Add<&FieldElement> for &FieldElement {
    type Output = FieldElement;
    fn add(self, other: &FieldElement) -> FieldElement {
        let result: BigInt = &self.value + &other.value;
        FieldElement::from_int(&self, result)
            % &FieldElement::from_int(&self, self.field_modulus.as_ref().clone())
    }
}

impl Sub<&FieldElement> for &FieldElement {
    type Output = FieldElement;
    fn sub(self, other: &FieldElement) -> FieldElement {
        let result: BigInt = &self.value - &other.value;
        FieldElement::from_int(&self, result)
            % &FieldElement::from_int(&self, self.field_modulus.as_ref().clone())
    }
}

impl Mul<&FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, other: &FieldElement) -> FieldElement {
        let result: BigInt = &self.value * &other.value;
        FieldElement::from_int(&self, result)
            % &FieldElement::from_int(&self, self.field_modulus.as_ref().clone())
    }
}

impl FieldElement {
    pub fn new(int: BigInt, field_modulus: BigInt) -> Self {
        FieldElement {
            value: int,
            field_modulus: Rc::new(field_modulus),
        }
    }
    fn from_int(&self, int: BigInt) -> Self {
        FieldElement {
            value: int,
            field_modulus: Rc::clone(&self.field_modulus),
        }
    }
    pub fn modpow(&self, exponent: BigInt) -> FieldElement {
        // (base ** exp) % n
        let field_modulus: BigInt = self.field_modulus.as_ref().clone();
        let result: BigInt = self.value.modpow(&exponent, &field_modulus);
        FieldElement::new(result, self.field_modulus.as_ref().clone())
    }
}

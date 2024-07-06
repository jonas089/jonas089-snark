use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::ops::{Add, Mul, Rem, Sub};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct FieldElement {
    pub value: Rc<BigInt>,
    pub field_modulus: Rc<BigInt>,
}

impl<'a> Rem<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn rem(self, other: &'a FieldElement) -> FieldElement {
        let result: BigInt = self.value.as_ref() % other.value.as_ref();
        if result < BigInt::zero() {
            return FieldElement {
                value: Rc::new(result + other.value.as_ref()),
                field_modulus: Rc::clone(&self.field_modulus),
            };
        };
        FieldElement {
            value: Rc::new(result),
            field_modulus: Rc::clone(&self.field_modulus),
        }
    }
}

impl<'a> Add<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn add(self, other: &'a FieldElement) -> FieldElement {
        let result: BigInt = self.value.as_ref() + other.value.as_ref();
        &FieldElement::from_int(&self, Rc::new(result))
            % &FieldElement::from_int(&self, Rc::clone(&self.field_modulus))
    }
}

impl<'a> Sub<&'a FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn sub(self, other: &'a FieldElement) -> FieldElement {
        let result: BigInt = self.value.as_ref() - other.value.as_ref();
        &FieldElement::from_int(&self, Rc::new(result))
            % &FieldElement::from_int(&self, Rc::clone(&self.field_modulus))
    }
}

impl<'a> Mul<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, other: &'a FieldElement) -> FieldElement {
        let result: BigInt = self.value.as_ref() * other.value.as_ref();
        &FieldElement::from_int(&self, Rc::new(result))
            % &FieldElement::from_int(&self, Rc::clone(&self.field_modulus))
    }
}

impl FieldElement {
    pub fn new(int: Rc<BigInt>, field_modulus: Rc<BigInt>) -> Self {
        FieldElement {
            value: int,
            field_modulus,
        }
    }
    fn from_int(&self, int: Rc<BigInt>) -> Self {
        FieldElement {
            value: int,
            field_modulus: Rc::clone(&self.field_modulus),
        }
    }
    pub fn modpow(&self, base: BigInt) -> FieldElement {
        // (base ** exp) % n
        let field_modulus: &BigInt = self.field_modulus.as_ref();
        let result: BigInt = self.value.modpow(&base, field_modulus);
        FieldElement::new(Rc::new(result), Rc::clone(&self.field_modulus))
    }
}

use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::Zero;

use crate::field::FieldElement;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
}

#[derive(Debug, Clone)]
pub struct Curve {
    pub a: BigInt,
    pub b: BigInt,
    pub p: BigInt,
}

impl Curve {
    #[allow(non_snake_case)]
    pub fn point_addition(&self, P: &Point, Q: &Point) -> Point {
        if (P.x.is_none()) && (P.y.is_none()) {
            return Q.clone();
        };
        if (Q.x.is_none()) && (Q.y.is_none()) {
            return P.clone();
        };
        let (x1, y1, x2, y2) = (
            P.x.clone().unwrap(),
            P.y.clone().unwrap(),
            Q.x.clone().unwrap(),
            Q.y.clone().unwrap(),
        );
        let mut m: FieldElement = FieldElement {
            value: Rc::new(BigInt::zero()),
            field_modulus: Rc::new(self.p.clone()),
        };
        if (x1.value == x2.value) && (y1.value == y2.value) {
            m = (&(FieldElement::new(Rc::new(BigInt::from(3)), Rc::new(self.p.clone()))
                * &x1
                * &x1)
                + &FieldElement::new(Rc::new(self.a.clone()), Rc::new(self.p.clone())))
                * (&(FieldElement::new(Rc::new(BigInt::from(2)), Rc::new(self.p.clone())) * &y1)
                    .modpow(self.p.clone() - 2));
        } else {
            m = (&y1 - &y2) * (&(&x2 - &x1).modpow(self.p.clone() - 2));
        };
        println!("M: {:?}", &m);
        let x3: FieldElement = &(&(m.clone() * &m) - &x1) - &x2;
        let y3: FieldElement = m * &(&(&x1 - &x3) - &y1);
        Point {
            x: Some(x3),
            y: Some(y3),
        }
    }
    #[allow(non_snake_case)]
    pub fn double_and_add(&self, n: &BigInt, P: &Point) -> Point {
        if P.x.is_none() && P.y.is_none() {
            return P.clone();
        };
        let mut temp_point: Point = Point {
            x: P.x.clone(),
            y: P.y.clone(),
        };
        let n_binary = format!("{:b}", n);
        for binary_char in n_binary.chars().skip(1) {
            temp_point = self.point_addition(&temp_point, &temp_point);
            if binary_char == '1' {
                temp_point = self.point_addition(&temp_point, &P);
            }
        }
        temp_point
    }
}

#[test]
fn test_generate_2g_for_secp256k1() {
    use num_traits::Num;
    let secp256k1: Curve = Curve {
        a: BigInt::zero(),
        b: b(),
        p: p(),
    };
    let point_g_2: Point = secp256k1.double_and_add(&2.into(), &g());
    println!("Point 2G: {:?}", &point_g_2);

    fn a() -> BigInt {
        BigInt::zero()
    }

    fn b() -> BigInt {
        BigInt::from(7u8)
    }

    fn p() -> BigInt {
        BigInt::from_str_radix(
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f",
            16,
        )
        .expect("Failed to construct BigInt from Hex")
    }

    fn gx() -> BigInt {
        BigInt::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .expect("Failed to construct BigInt from Hex")
    }
    fn gy() -> BigInt {
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
}

#[test]
fn ecdsa(){
    use num_bigint_dig::traits::ModInverse;
}
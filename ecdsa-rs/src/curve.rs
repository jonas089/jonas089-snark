use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::Zero;

use crate::field::FieldElement;
mod secp256k1;

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
            m = (&y2 - &y1) * &((&x2 - &x1).modpow(self.p.clone() - 2));
        };
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

#[cfg(test)]
mod tests {
    use super::{
        secp256k1::{b, g, p},
        BigInt, Curve, FieldElement, Point, Rc,
    };
    use num_traits::{One, Zero};
    #[test]
    fn test_generate_2g_for_secp256k1() {
        let secp256k1: Curve = secp256k1_init();
        let point_g_2: Point = secp256k1.double_and_add(&2.into(), &g());
        println!("Point 2G: {:?}", &point_g_2);
    }

    #[test]
    fn ecdsa() {
        use crate::curve::secp256k1;
        use secp256k1::g;
        let secp256k1 = secp256k1_init();
        // 2p + 3 as k
        let k: BigInt = 3.into();
        let kG: Point = secp256k1.double_and_add(&k, &g());
        println!("kG: {:?}", &kG);
        return;
        let d: BigInt = 20.into();
        let dG: Point = secp256k1.double_and_add(&d, &g());
        let r: FieldElement = kG.x.clone().unwrap();
        let h: BigInt = 20.into();
        let k_inverse: BigInt = modinv(k.clone(), secp256k1.p.clone());
        // k_inverse * (h + r * p)
        let s: FieldElement = FieldElement::new(Rc::new(k_inverse), Rc::new(secp256k1.p.clone()))
            * &(&FieldElement::new(Rc::new(h.clone()), Rc::new(secp256k1.p.clone()))
                + &(FieldElement::new(
                    Rc::new(r.value.as_ref().clone()),
                    Rc::new(secp256k1.p.clone()),
                ) * &FieldElement::new(Rc::new(d), Rc::new(secp256k1.p.clone()))));
        assert!(r.value.as_ref() < &secp256k1.p);
        assert!(s.value.as_ref() < &secp256k1.p);
        // R' = (h * s1) * G + (r * s1) * public_key
        let s_inverse: BigInt = modinv(s.value.as_ref().clone(), secp256k1.p.clone());
        assert!(&s_inverse < &secp256k1.p);
        // assert correctness of modular inverse
        assert_eq!(
            s.value.as_ref().clone() * s_inverse.clone() % secp256k1.p.clone(),
            1.into()
        );
        
        let sh: FieldElement = FieldElement::new(Rc::new(h), Rc::new(secp256k1.p.clone()))
            * &FieldElement::new(Rc::new(s_inverse.clone()), Rc::new(secp256k1.p.clone()));
        let shg: Point = secp256k1.double_and_add(sh.value.as_ref(), &g());

        let sr: FieldElement =
            r.clone() * &FieldElement::new(Rc::new(s_inverse), Rc::new(secp256k1.p.clone()));
        let srp: Point = secp256k1.double_and_add(sr.value.as_ref(), &dG);
        
        let verifier: Point = secp256k1.point_addition(&shg, &srp);
        assert_eq!(r.value.as_ref(), verifier.x.unwrap().value.as_ref());
    }

    fn secp256k1_init() -> Curve {
        Curve {
            a: BigInt::zero(),
            b: b(),
            p: p(),
        }
    }

    fn modinv(a0: BigInt, m0: BigInt) -> BigInt {
        let mut a = a0;
        let mut m = m0.clone();
        let mut x0 = BigInt::zero();
        let mut inv = BigInt::one();

        while a > BigInt::one() {
            let quotient = &a / &m;
            let remainder = &a % &m;
            a = m;
            m = remainder;
            let new_inv = &inv - &quotient * &x0;
            inv = x0;
            x0 = new_inv;
        }
        if inv < BigInt::zero() {
            inv += m0;
        }
        inv
    }
}

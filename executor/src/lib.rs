use num_bigint::BigInt;

use rust_ec::curve::{params,{Curve, Point}};
use rust_ec::curve::modulo;
use params::{secp256k1_a, secp256k1_b, secp256k1_g, secp256k1_p, test_g};

#[test]
fn addition_program(){
    let g: Point = secp256k1_g();
    let a = BigInt::from(1u8);
    let b = BigInt::from(2u8);
    let c = BigInt::from(3u8);

    let curve = Curve{
        a: secp256k1_a(),
        b: secp256k1_b(),
        p: secp256k1_p()
    };

    let a_g = curve.double_and_add(&a, &g);
    let b_g = curve.double_and_add(&b, &g);
    let c_g = curve.double_and_add(&c, &g);

    let sum = curve.point_addition(&a_g, &b_g);
    println!("cG: {:?}, aG + bG: {:?}", &c_g, &sum);
}

#[test]
fn pseudo_multiplication_program(){
    let a = BigInt::from(2u8);
    let b = BigInt::from(3u8);
    let c = BigInt::from(6u8);

    let curve = Curve{
        a: secp256k1_a(),
        b: secp256k1_b(),
        p: secp256k1_p()
    };

    let pairing_prover = pseudo_pairing(&a, &b, &curve);
    let pairing_verifier = curve.double_and_add(&c, &secp256k1_g());
   
    println!("prover: {:?}, verifier: {:?}", &pairing_prover, &pairing_verifier);
    /// This is not a real pairing function and does not obfuscate a or b
    /// aG -> double G a times
    /// bG -> double G b times
    /// cG -> double G c times
    /// a * b = c
    /// aG * bG != cG
    /// aG * bG = (a+b)G
    /// f(P, Q) = R

    fn pseudo_pairing(a: &BigInt, b: &BigInt, curve: &Curve) -> Point{
        let scalar = a * b;
        curve.double_and_add(&scalar, &secp256k1_g())
    }
}
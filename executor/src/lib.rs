use num_bigint::BigInt;

use rust_ec::curve::{params,{Curve, Point}};
use rust_ec::curve::modulo;
use params::{secp256k1_a, secp256k1_b, secp256k1_g, secp256k1_p, test_g};

#[test]
fn addition_program(){
    let G: Point = secp256k1_g();

    let a = BigInt::from(1u8);
    let b = BigInt::from(2u8);
    let c = BigInt::from(3u8);

    let curve = Curve{
        a: secp256k1_a(),
        b: secp256k1_b(),
        p: secp256k1_p()
    };

    let aG = curve.double_and_add(&a, &G);
    let bG = curve.double_and_add(&b, &G);
    let cG = curve.double_and_add(&c, &G);

    let sum = curve.point_addition(&aG, &bG);
    println!("cG: {:?}, aG + bG: {:?}", &cG, &sum);
}

#[test]
fn multiplication_program(){
    let G: Point = secp256k1_g();
    let G2: Point = test_g();

    let a = BigInt::from(2u8);
    let b = BigInt::from(3u8);
    let c = BigInt::from(6u8);

    let curve = Curve{
        a: secp256k1_a(),
        b: secp256k1_b(),
        p: secp256k1_p()
    };

    let aG = curve.double_and_add(&a, &G);
    let bG2 = curve.double_and_add(&b, &G2);
    let cG = curve.double_and_add(&c, &G);

    let pairing_prover = pairing(&aG, &bG2, &curve);
    let pairing_verifier = pairing(&cG, &G2, &curve);
   
    println!("prover: {:?}, verifier: {:?}", &pairing_prover, &pairing_verifier);
    fn pairing(a: &Point, b: &Point, curve: &Curve) -> Point{
        let ax = a.x.clone().unwrap();
        let bx = b.x.clone().unwrap();
        let abx = modulo(&(ax*bx), &secp256k1_p());
        curve.double_and_add(&abx, &secp256k1_g())
    }
}
use std::str::FromStr;

use num_bigint::BigInt;

use num_traits::One;
use vanilla_ecc::curve::{params::{self, two},Curve, Point};
use vanilla_ecc::curve::modulo;
use params::Secp256k1;

#[test]
/* Additon obfuscation
    a -> aG, 
    b -> bG,
    ...,
    n -> nG

    a + b = c,
    aG + bG = cG

    Verifier can compute cG and check if aG and bG provided by the prover equal cG.
    If this is the case then the prover knows private values a, b such that a + b = c
*/
fn addition_program(){
    let secp256k1 = Secp256k1;
    let g: Point = secp256k1.g();
    let a = BigInt::from(1u8);
    let b = BigInt::from(2u8);
    let c = BigInt::from(3u8);

    let curve = Curve{
        a: secp256k1.a(),
        b: secp256k1.b(),
        p: secp256k1.p()
    };

    let a_g = curve.double_and_add(&a, &g);
    let b_g = curve.double_and_add(&b, &g);
    let c_g = curve.double_and_add(&c, &g);

    let sum = curve.point_addition(&a_g, &b_g);
    assert_eq!(&c_g.x, &sum.x);
    assert_eq!(&c_g.y, &sum.y);
}

#[test]
/* Pseudo Pairing
    This test does not perform an actual bilinear pairing.
    Therefore it is sadly not yet possible to obfuscate inputs in multiplication gates.
    A real pairing algorithm and a pairing friendly curve must be used to achieve obfuscation.
*/
fn pseudo_multiplication_program(){
    let secp256k1 = Secp256k1;
    let a = BigInt::from(2u8);
    let b = BigInt::from(3u8);
    let c = BigInt::from(6u8);

    let curve = Curve{
        a: secp256k1.a(),
        b: secp256k1.b(),
        p: secp256k1.p()
    };

    let pairing_prover = pseudo_pairing(&a, &b, &secp256k1.g(), &curve);
    let pairing_verifier = curve.double_and_add(&c, &secp256k1.g());

    println!("prover: {:?}, verifier: {:?}", &pairing_prover, &pairing_verifier);
    assert_eq!(&pairing_prover.x, &pairing_verifier.x);
    assert_eq!(&pairing_prover.y, &pairing_verifier.y);
    /// aG -> double_and_add(a, G)
    /// bG 
    /// cG 
    /// a * b = c
    /// aG * bG != cG
    /// aG * bG = (a*b)G

    /// This is not a real pairing function and does not obfuscate a or b
    fn pseudo_pairing(a: &BigInt, b: &BigInt, g: &Point, curve: &Curve) -> Point{
        let scalar = a * b;
        curve.double_and_add(&scalar, g)
    }
}
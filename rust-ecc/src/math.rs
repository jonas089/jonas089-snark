use num_bigint::BigInt;
use num_traits::{One, Zero};
// a modulo function that behaves similar to the Python % operator
pub fn modulo(a: &BigInt, p: &BigInt) -> BigInt {
    let result = a % p;
    if result < BigInt::zero() {
        result + p
    } else {
        result
    }
}

pub fn prime_field_inv(mut a: BigInt, n: BigInt) -> BigInt {
    // Extended euclidean algorithm to find modular inverses for integers
    a = modulo(&a, &n);
    if a == BigInt::zero() {
        return BigInt::zero();
    }
    let (mut lm, mut hm) = (BigInt::one(), BigInt::zero());
    let (mut low, mut high) = (modulo(&a, &n), n.clone());
    while low > BigInt::one() {
        let r = &high / &low;
        let (nm, new) = (&hm - &lm * &r, high - &low * &r);
        hm = lm;
        lm = nm;
        high = low;
        low = new;
    }
    modulo(&lm, &n)
}

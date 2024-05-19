use num_traits::Zero;
use num_bigint::BigInt;

// a modulo function that behaves similar to the Python % operator
pub fn modulo(a: &BigInt, p: &BigInt) -> BigInt {
    let result = a % p;
    if result < BigInt::zero() {
        result + p
    } else {
        result
    }
}

/*// Calculates the modular inverse of `a` modulo `p` using Fermat's Little Theorem
pub fn modular_inverse(a: &BigInt, p: &BigInt) -> BigInt {
    if a.is_zero() {
        // Typically there is no modular inverse for 0, handling this based on your application's needs
        return Zero::zero();
    }
    // Fermat's Little Theorem: a^(p-2) % p
    let exponent = p - 2u32.to_bigint().unwrap();
    a.modpow(&exponent, p)
}*/
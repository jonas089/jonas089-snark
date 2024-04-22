# Rust-ec - elliptic curve operations in Rust

This library exposes an api to construct elliptic curves such as secp256k1 and perform point doubling and point addition

# Examples

see tests in `lib.rs`:

```rust
    use params::{secp256k1_a, secp256k1_b, secp256k1_p, secp256k1_g};
    let curve: Curve = Curve{
        a: secp256k1_a(),
        b: secp256k1_b(),
        p: secp256k1_p()
    };
    let point_2g: Point = curve.double_and_add(&BigInt::from(2u8), &secp256k1_g());
    // Todo: make this an assertion and expand on the tests
    println!("Point 2G: {:?}", &point_2g);
```

# Vanilla-ecc

This library exposes an api to construct elliptic curves such as secp256k1 and perform point doubling and point addition

# Examples

see tests in `curve.rs`:

```rust
fn addition_program(){
    let secp256k1 = SECP_256_K1{};
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
```

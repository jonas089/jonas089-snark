# Arithmetic proofs from scratch
Jsnark is a passion project of @jonas089 that strives to be a minimum viable SNARK proving system for simple arithmetic circuits, implemented entirely in Rust and from scratch.

If you wish to contribute or help me implement bilinear pairings feel free to reach out via [LinkedIn](https://www.linkedin.com/in/jonas-pauli/) or raise an issue directly here on this git repository. Any help is welcome :).

Currently the goal is to be able to prove addition and multiplication circuits to then later be able to generate proofs of computation for simple arithmetic circuits with only multiplication and addition gates.

I already implemented a basic addition program on top of the my custom `curve` library using the `secp-256k1` curve, multiplication however requires bilinear pairings and a pairing friendly curve which is yet to be done.

# Example: Basic addition program over secp256k1

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

This addition program enables obfuscation of a, b and can be used to prove that `aG + bG == cG`, where c is known to the verifier and a, b are only known to the prover.

In cases where either a or b are public inputs, they can be committed to some public journal.

Modern SNARKs involve additional steps, but for my prototype this is a deemed a minimum viable means of proving addition.



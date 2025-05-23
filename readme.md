>[!WARNING] 
> This library has not been audited and should not be used in production.
>
> Some of the features are unfinished as this is a purely educational/research project.

# Arithmetic proofs from scratch

Jsnark is a passion project of @jonas089 that strives to be a minimum viable SNARK proving system for simple arithmetic circuits, implemented entirely in Rust and from scratch.

If you wish to contribute or help me implement bilinear pairings feel free to reach out via [LinkedIn](https://www.linkedin.com/in/jonas-pauli/) or raise an issue directly here on this git repository. Any help is welcome :).

Currently the goal is to be able to prove addition and multiplication circuits to then later be able to generate proofs of computation for simple arithmetic circuits with only multiplication and addition gates.

I already implemented a basic addition program on top of the my custom `curve` library using the `secp-256k1` curve, multiplication however requires bilinear pairings and a pairing friendly curve which is yet to be done.

# Running Tests
```
cargo test -p executor
```

# Vanilla-ecc vs ecdsa-rs
ecdsa-rs is a refactor of Vanilla-ecc. Vanilla-ecc was designed with Curves in mind, whilst ecdsa-rs was designed with Fields in mind. Therefore ecdsa-rs is closer to what one would see in production (e.g. Ethereum's py-ecc).

For educational purposes it can make sense to review both, but ecdsa-rs is probably easier to understand for most people. When I first learned about ECC I didn't know much about fields and therefore ended up implementing Vanilla-ecc.

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

# Bilinear pairings
I have started implementing `rust-rs` which is largely inspired by the Ethereum Foundation's `rust-ecc`. The crate `vanilla-ecc` was designed with curve-orientation, which turned out to not be suitable for the multidimensional elliptic curve operations that are required for bilinear pairings.

`vanilla-ecc` can only be used to obfuscate inputs in addition circuits and for regular asymmetric key cryptography.

`ecdsa-rs` will hopefully soon provide similar functionality to `py-ecc` with bilinear pairings and zk-friendly curves.


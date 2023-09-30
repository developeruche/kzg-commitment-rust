










## Little Introduction to KZG commitment

KZG commitment is a type of polynomial commitment scheme that is very efficient and secure. It is based on the bilinear pairing, which is a special type of map between two elliptic curve groups.

Polynomial commitment scheme is a cryptographic scheme that allows a prover to commit to a polynomial in a way that hides the polynomial from the verifier, but allows the verifier to verify that the prover knows the polynomial and that the prover cannot change the polynomial after committing to it.

How KZG commitment works
To commit to a polynomial using KZG commitment, the prover first generates a public parameter. The public parameter contains a vector of points in the G1 group and a single point in the G2 group.

The prover then calculates a commitment to the polynomial in the G2 group using the public parameter and the polynomial coefficients. The commitment is then returned to the verifier along with the public parameter.

To verify the commitment, the verifier simply needs to compute the same commitment to the polynomial using the public parameter. If the two commitments are equal, then the verifier can be confident that the prover knows the polynomial and that the commitment is valid.

Advantages of KZG commitment
KZG commitment has a number of advantages over other polynomial commitment schemes, including:

Efficiency: KZG commitment is very efficient, both in terms of computation and communication.
Security: KZG commitment is very secure, and it is resistant to a variety of attacks.
Flexibility: KZG commitment can be used to commit to polynomials of any degree.
Applications of KZG commitment
KZG commitment is used in a variety of applications, including:

Zero-knowledge proofs: KZG commitment can be used to construct zero-knowledge proofs, which are proofs that allow a prover to convince a verifier that a statement is true without revealing any additional information to the verifier.
Accumulators: KZG commitment can be used to construct accumulators, which are data structures that allow a prover to efficiently prove that a value is included in a set.
Blockchain scaling: KZG commitment can be used to scale blockchain networks by allowing proofs to be verified more efficiently.
Conclusion
KZG commitment is a powerful and versatile cryptographic tool. It is used in a variety of applications, including zero-knowledge proofs, accumulators, and blockchain scaling.
# Chaum-Pedersen Protocol for Equality of Discrete Logarithms

This repository contains an experimental implementation of the Chaum-Pedersen protocol in Rust for proving the equality of discrete logarithms as a zero-knowledge proof.

## Overview

The Chaum-Pedersen protocol is a specific instance of a Sigma protocol designed to prove that two discrete logarithms are equal without revealing the actual values of the logarithms themselves. It is commonly used in cryptographic applications where proving knowledge of discrete logarithms is required while preserving privacy.

This implementation provides a basic framework for executing the Chaum-Pedersen protocol in Rust. It includes functionality for generating commitments, computing responses, and verifying the proofs. The protocol relies on the properties of modular exponentiation and uses BigIntegers for arbitrary-precision arithmetic.

## Work in Progress

**This project is experimental and a work in progress.** It may contain bugs, incomplete features, or unoptimized code. Use it at your own risk and discretion.

## Contributing

Contributions to this project are welcome! If you find any bugs or have suggestions for improvements, please open an issue or submit a pull request. Make sure to follow the project's coding conventions and guidelines.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

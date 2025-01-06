# Multiple Mutable Borrows in Rust

This repository demonstrates a common error in Rust programming: attempting to create multiple mutable borrows of the same variable.  Rust's ownership system prevents data races by enforcing strict rules about borrowing.

The `bug.rs` file contains code that tries to violate these rules, resulting in a compilation error. The `bugSolution.rs` file shows how to correctly handle the situation using techniques such as cloning, immutability, or refactoring.

This example highlights the importance of understanding Rust's borrowing system to avoid common pitfalls and write safe, concurrent code.
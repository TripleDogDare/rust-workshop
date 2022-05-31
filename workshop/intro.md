---
marp: true
---

# Rust Workshop Series

## Knox Networks

Follow along, clone https://github.com/knox-networks/rust-sdk

---

# Resources

- [Install Rust Toolchain](https://www.rust-lang.org/tools/install)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Learn Rust](https://www.rust-lang.org/learn)
- [Are We ... Yet](https://wiki.mozilla.org/Areweyet)
- [Rust in Action](https://www.manning.com/books/rust-in-action)
- [Rust Playground](https://play.rust-lang.org/)

---

# Workspace

```toml
[workspace]
members = [
    "identity"
]
```

```
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE-APACHE
├── LICENSE-MIT
├── identity
├── rust-toolchain
├── target
└── workshop
```

---

# Your First Rust Library

`cargo new [name] --lib`

```toml
[package]
name = "knox-identity"
version = "0.1.0"
edition = "2021"
description = "Knox Networks Rust Identity SDK"
license = "MIT OR Apache-2.0"
authors = ["developers@knox-networks.com"]

[[bin]]
path = "bin/vc_adapter.rs"
name = "knox-vc-adapter"

[dependencies]
serde = "1.0.137"
serde_json = "1.0.81"
thiserror = "1.0.31"
monetae-sdk = { git = "https://github.com/monetae/core" }
```

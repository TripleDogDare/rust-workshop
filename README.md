Workshops:
1. https://www.rust-lang.org/learn/get-started
2.

## Resources

* [Go ⃕ Rust patterns](https://programming-idioms.org/cheatsheet/Go/Rust)
* [Javascript ⃕ Rust patterns](https://programming-idioms.org/cheatsheet/JS/Rust)
* [VSCode rust-analyzer](https://code.visualstudio.com/docs/languages/rust)
* [Install Rust Toolchain](https://www.rust-lang.org/tools/install)
* [The Rust Book](https://doc.rust-lang.org/book/)
* [Learn Rust](https://www.rust-lang.org/learn)
* [Rust Playground](https://play.rust-lang.org/)
* [cheats.rs](https://cheats.rs)
* [rust-concurrency](https://github.com/quambene/rust-concurrency)

## Quickref:

* `dbg!()` macro

```rust
let a = 2;
let b = dbg!(a * 2) + 1;
//      ^-- prints: [src/main.rs:2] a * 2 = 4
assert_eq!(b, 5);
```

* `cargo test -- --nocapture`

```rust
#[test]
fn test() {
    println!("If `cargo test --` gets the `--nocapture` flag, \
              then I will print to stdout");
}
```

# Resources


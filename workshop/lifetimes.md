---
paginate: false
---

# Lifetimes


## Two main pillars of lifetimes

* AXM
* RAII

---

[aliasing XOR mutability](http://plv.mpi-sws.org/rustbelt/ghostcell/paper.pdf)

>  Rust’s type system enforces the discipline of aliasing _XOR_ mutability
>  (AXM, for short): a value of type `T` may either have *multiple aliases*
>  (called shared references), of type `&T`, or it may be mutated via a
>  *unique, mutable reference*, of type `&mut T`, but it *may not be both
>  aliased and mutable at the same time*

---

[RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)

> Resource acquisition is initialization (RAII) is a programming idiom used
> in several object-oriented, statically-typed programming languages to describe
> a particular language behavior. In RAII, holding a resource is a class
> invariant, and is tied to object lifetime. Resource allocation (or acquisition)
> is done during object creation (specifically initialization), by the
> constructor, while resource deallocation (release) is done during object
> destruction (specifically finalization), by the destructor. In other words,
> resource acquisition must succeed for initialization to succeed. Thus the
> resource is guaranteed to be held between when initialization finishes and
> finalization starts (holding the resources is a class invariant), and to be
> held only when the object is alive. Thus if there are no object leaks, there
> are no resource leaks.

Invariant - a logical assertion that is always held to be true during a certain
phase of execution of a computer program

---

[Borrowing](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html)

```rust
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+

```

---

[Elision](https://doc.rust-lang.org/rust-by-example/scope/lifetime/elision.html)

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

---

[Mutability](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

---

[Exclusive Mutability](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)

```rust
    let mut s = String::from("hello");

    let r1 = &s;     // no problem
    let r2 = &s;     // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");
```

#### Listing 10-18: Annotations of the lifetimes of r and x, named 'a and 'b, respectively

# Resources
[Lifetimes](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html)
[Cargo Expand](https://github.com/dtolnay/cargo-expand)

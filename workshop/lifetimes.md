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

```rust
// raii.rs
fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3);

    // `_box1` is destroyed here, and memory gets freed
}

fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}
```

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

Annotations of the lifetimes of `r` and `x`, named `'a` and `'b`, respectively

---

[Elision](https://doc.rust-lang.org/rust-by-example/scope/lifetime/elision.html)

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

---

```rust
fn print1(s: &str);                                   // elided
fn print2(s: &'_ str);                                // also elided
fn print3<'a>(s: &'a str);                            // expanded

fn debug1(lvl: usize, s: &str);                       // elided
fn debug2<'a>(lvl: usize, s: &'a str);                // expanded

fn substr1(s: &str, until: usize) -> &str;            // elided
fn substr2<'a>(s: &'a str, until: usize) -> &'a str;  // expanded

fn get_mut1(&mut self) -> &mut dyn T;                 // elided
fn get_mut2<'a>(&'a mut self) -> &'a mut dyn T;       // expanded

fn args1<T: ToCStr>(&mut self, args: &[T]) -> &mut Command;                  // elided
fn args2<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

fn new1(buf: &mut [u8]) -> Thing<'_>;                 // elided - preferred
fn new2(buf: &mut [u8]) -> Thing;                     // elided
fn new3<'a>(buf: &'a mut [u8]) -> Thing<'a>;          // expanded

type FunPtr1 = fn(&str) -> &str;                      // elided
type FunPtr2 = for<'a> fn(&'a str) -> &'a str;        // expanded

type FunTrait1 = dyn Fn(&str) -> &str;                // elided
type FunTrait2 = dyn for<'a> Fn(&'a str) -> &'a str;  // expanded
```

---

[Mutability](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html)

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

---

A regular reference (`&`) is a type of pointer, and one way to think of a pointer is
as an arrow to a value stored somewhere else. In the snippet below, we create a
reference to an `i32` value and then use the dereference operator to follow the
reference to the data:

Filename: src/main.rs

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
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

---

Anatomy of a rust function:

```rust
pub fn some_func<'lifetime, Type>(parameter: &'lifetime Type) -> Wrapper<&'lifetime Type>
where Type: Debug
{
    Wrapper::new(parameter)
}
```

---

# Resources
[Lifetimes](https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html)
[Cargo Expand](https://github.com/dtolnay/cargo-expand)

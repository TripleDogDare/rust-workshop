---
marp: true
---

# Rust Workshop Series

## Knox Networks

### Workshop Pt. 3: Generics and Traits

---

# Generic Types

#

> In Rust, "generic" ... describes anything that accepts one or more generic type parameters `<T>`. Any type specified as a generic type parameter is generic, and everything else is concrete (non-generic).

<small>https://doc.rust-lang.org/rust-by-example/generics.html</small>

---

# Generics in Functions

```rust
// Generic types are defined using <T> following the function name
fn foo<T>(arg: T) { ... }

// Generic types can be borrowed
fn bar<T>(arg: &T) { ... }

// Generic types can be name anything, but typically are a single character
fn fizz<P>(arg: &P) { ... }

// Generic types can be returned
fn buzz<U>() -> U { ... }

// Multiple generic types can be used
fn far<T,U>(arg: T) -> U { ... }

// Generic types can have lifetimes
fn out<'a,T,U>(arg: &'a T) -> U { ... }
```

---

# Generics in Structs

```rust
// Generic types can be used to define multiple fields of the same type
struct Point<T> {
    x: T,
    y: T
}

// Multiple generic types are needed to define different typed fields
enum Result<T,E> {
    Ok(T),
    Err(E)
}
```

---

# Traits

> A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

> _Note: Traits are similar to a feature often called interfaces in other languages, although with some differences._

<small>https://doc.rust-lang.org/book/ch10-02-traits.html</small>

---

# Generics in Traits

```rust
// Traits can also have generics, and define default types
trait Add<T = Self> {
    // traits can have internal generic types, referenced by `Self::...`
    type Output;

    fn add(self, other: T) -> Self::Output;
}

// Traits with internal types are defined as trait bounds
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

---

# Trait Bounds

```rust
pub trait ToString {
    fn to_string(self) -> String;
}

// The generic type `T` has a trait bound of `ToString`
pub fn foo<T: ToString>(arg: T) -> String {
    arg.to_string()
}

// Syntactic sugar for the equivalent above
pub fn foo(arg: impl ToString) -> String {
    arg.to_string()
}

// Generic types can have multiple trait bounds
pub fn foo<T: ToString + Add<Output = T>>(arg: T, other: T) -> String {
    arg.add(other).to_string()
}

// Use the `where` clause to keep your trait bounds more readable
pub fn foo<T>(arg: T, other: T) -> String
    where T: ToString + Add<Output = T>,
{
    arg.add(other).to_string()
}

```

---

# Monomorphization

> You might be wondering whether there is a runtime cost when using generic type parameters. The good news is that using generic types won't make your run any slower than it would with concrete types.

> Rust accomplishes this by performing monomorphization of the code using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

<small>https://doc.rust-lang.org/book/ch10-01-syntax.html</small>

---

# Dynamic Types

> The dyn keyword is used to highlight that calls to methods on the associated Trait are dynamically dispatched. To use the trait this way, it must be ‘object safe’.

> Unlike generic parameters or impl Trait, the compiler does not know the concrete type that is being passed. That is, the type has been erased. As such, a dyn Trait reference contains two pointers. One pointer goes to the data (e.g., an instance of a struct). Another pointer goes to a map of method call names to function pointers (known as a virtual method table or vtable).

> At run-time, when a method needs to be called on the dyn Trait, the vtable is consulted to get the function pointer and then that function pointer is called.

<small>https://doc.rust-lang.org/std/keyword.dyn.html</small>

---

```rust

// Due to monomorphization, functions can return types that implement a trait,
// but only of one concrete value
pub fn foo() -> impl ToString { ... }

// To enable a function to return multiple types,
// each implementing a trait, we need to use a smart pointer
// and dynamic dispatch
pub fn bar() -> Vec<Box<dyn ToString>> { ... }
```

> The above indirection is the additional runtime cost of calling a function on a dyn Trait. Methods called by dynamic dispatch generally cannot be inlined by the compiler.

> However, dyn Trait is likely to produce smaller code than impl Trait / generic parameters as the method won’t be duplicated for each concrete type.

<small>https://doc.rust-lang.org/std/keyword.dyn.html</small>


Seems like we need a main function to kick things off.
```Rust
fn main() {
    // Dump crap here
}
```

`println!` is a macro. We don't know what that is yet, but this is the standard printing function.
```Rust
println!("Hello World");
```

This does not work, the print macro requires at least 1 parameter:
```Rust
println!();
```

Functions do not seem to need to be declared, only defined.

So here's where C comes into play. We have 2 types of strings here. One of them is the constant string and one of them is the not so constant one. 

Test with assert?

```Rust
fn assert_strings() {
    let hello_world = "Hello World!";
    println!("{}", (assert!("Hello World!" == hello_world)).to_string());
    println!("{}", (assert!("Hello World!" != hello_world)).to_string());
}
```

Lol. Nope. Returns:

```
()
task '<main>' panicked at 'assertion failed: "Hello World!" != hello_world', hello_exp.rs:20
```

Maybe I should wait until I get smarter.

...

Okay, I've dechipered it. This seems to be fine:
```Rust
fn assert_strings() {
    let hello_world = "Hello World!";
    assert!("Hello World!" != hello_world);
}
```
Returns:
```
task '<main>' panicked at 'assertion failed: "Hello World!" != hello_world', hello_exp.rs:20
```

This means that the assertion is true on the two strings... Are the types the same?

Looked it up, currently there's no way to check the type of a variable in this language. Pretty sad thing, but let's see the workarounds:

```Rust
let hello_world = "Hello World!";
let type_hello: () = hello_world;
```
```
hello_exp.rs:26:26: 26:37 error: mismatched types: expected `()`, found `&str` (expected (), found &-ptr)
hello_exp.rs:26     let type_hello: () = hello_world;
```

```Rust
let type_hello: () = "Hello World!";
```
```
hello_exp.rs:26:26: 26:40 error: mismatched types: expected `()`, found `&'static str` (expected (), found &-ptr)
hello_exp.rs:26     let type_hello: () = "Hello World!";
```

My work is done here. I should advance.
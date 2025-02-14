# Orphan Fool

A hack to try disabling the orphan rule in rust.

## Demo

```rust
use std::fmt;

impl fmt::Display for Vec<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn main() {
    let x = vec![1, 2, 3];

    println!("x = {x}");
}
```

Normally this wouldn't be allowed.

```
$ rustc ./demo.rs 
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
 --> ./demo.rs:3:1
  |
3 | impl fmt::Display for Vec<i32> {
  | ^^^^^^^^^^^^^^^^^^^^^^--------
  |                       |
  |                       `Vec` is not defined in the current crate
  |
  = note: impl doesn't have any local type before any uncovered type parameters
  = note: for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
  = note: define and implement a trait or new type instead

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0117`.
```

However, with the power of overriding queries, we can make this work:

```
$ cargo run ./demo.rs 
   Compiling orphan-fool v0.1.0 (/home/alona/dev/rust/orphan-fool)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.34s
     Running `target/debug/orphan-fool ./demo.rs`
alona@Ashtabula:~/dev/rust/orphan-fool$ ./demo 
x = [1, 2, 3]
```

## Limitations

I've not tested this with multiple crates.

## Contributing

Don't! This was hacked up in an evening to see if it'd work.

## Credits

Based on <https://jyn.dev/rustc-driver/> and <https://github.com/thomcc/ubrustc>

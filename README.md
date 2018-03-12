Ferris print
============
[![Build Status](https://travis-ci.org/kimond/ferris-print.svg?branch=master)](https://travis-ci.org/kimond/ferris-print)

A simple macro to print using ferris say.

## Why?
Sometime I am bored with the classic `println!`. But with `ferrisprint!` my output is more exiting.

## Instruction
Put the following in you `Cargo.toml`:

```toml
[dependencies]
ferris_print = "0.1"
```

Then import the crate with:

```rust
#[macro_use]
extern crate ferris_print;
```

### Example

```rust
#[macro_use]
extern crate ferris_print;

fn main() {
    ferrisprint!("Hello world");
}
```

The code above will print out this:

```plain
----------------------------
| Hello world              |
----------------------------
              \
               \
                  _~^~^~_
              \) /  o o  \ (/
                '_   -   _'
                / '-----' \
```
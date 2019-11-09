Ferris print
============
[![Build Status](https://travis-ci.org/kimond/ferris-print.svg?branch=master)](https://travis-ci.org/kimond/ferris-print)

A simple macro to print using ferris say.

## Why?
Sometime I am bored with the classic `println!`. But with `ferrisprint!` my output is more exciting.

## Instruction
Put the following in you `Cargo.toml`:

```toml
[dependencies]
ferris_print = "0.1"
```

Then import the crate with:

```rust
use ferris_print::ferrisprint;
```

### Example

```rust
use ferris_print::ferrisprint;

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

## Credits
* [@mgattozzi](https://github.com/mgattozzi) for [ferris-say](https://github.com/mgattozzi/ferris-says)

# easy_switch

A macro to emulate switch statements in C-style languages. Get rid of those long
`if`/`else if` chains!

# Syntax

Use the `switch!` macro to get started! This will look like a `match` expression,
but of course does no actual pattern matching.

```rust
use easy_switch::switch;

#[derive(PartialEq, Eq)]
struct Example {
    field: i32,
    field2: i32,
}

impl Example {
    pub fn new(field2: i32) -> Self {
        Self {
            field: 10,
            field2,
        }
    }
}

let switchable = Example::new(10);
let result = switch! { switchable;
    Example::new(50), 50 == 50 => 50,
    Example::new(20), 50 == 50, 20 == 20 => 20,
    _ => 0,
};
assert_eq!(0, result);
```

Check out [the docs](https://docs.rs/easy_switch) for more information on this
macro.

# License

This crate is licensed under the
[MIT](https://github.com/dzfrias/easy_switch/LICENSE) license.

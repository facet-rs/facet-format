# facet-value-format

<!-- cargo-reedme: start -->

<!-- cargo-reedme: info-start

    Do not edit this region by hand
    ===============================

    This region was generated from Rust documentation comments by `cargo-reedme` using this command:

        cargo +nightly reedme --workspace

    for more info: https://github.com/nik-rev/cargo-reedme

cargo-reedme: info-end -->

Serialize any type implementing `Facet` into a [`facet_value::Value`](https://docs.rs/facet_value/latest/facet_value/value/struct.Value.html).

This crate hosts the adapter between `facet-format`’s event serializer model
and `facet-value`’s dynamic `Value` type.

## Example

```rust
use facet::Facet;
use facet_value::{Value, from_value};
use facet_value_format::to_value;

#[derive(Debug, Facet, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

let person = Person { name: "Alice".into(), age: 30 };
let value: Value = to_value(&person).unwrap();

let person2: Person = from_value(value).unwrap();
assert_eq!(person, person2);
```

<!-- cargo-reedme: end -->

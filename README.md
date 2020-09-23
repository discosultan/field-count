# field_count

Derive the field count for a struct. Implements a `FieldCount` trait. Supports generic structs.

## 📦 Getting Started

```toml
# Cargo.toml

[dependencies]
field_count = "0.1"
```

```rust
// main.rs

use field_count::FieldCount;

#[derive(FieldCount)]
struct MyStruct {
    first_field: i32,
    second_field: String,
    third_field: u16,
}

fn main() {
    println!("{}", MyStruct::field_count()); // 3
}
```

## 🙏 Credits

This crate was inspired by [the following StackOverflow answer](https://stackoverflow.com/a/54177920/1466456) by [Lukas Kalbertodt](https://github.com/LukasKalbertodt).

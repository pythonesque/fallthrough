# Fallthrough

A macro providing fallthrough `match`.

## Requirements

Include the following in your `Cargo.toml`:

```toml
[dependencies.fallthrough]
git = "https://github.com/pythonesque/fallthrough"
version = "0.0.1"
```

In your `lib.rs`:

```rust
#[macro_use] extern crate fallthrough;
```

## Usage

```rust
#[allow(unreachable_code)]
fn main() {
    let mut x = 0;

    match_fallthrough!(x, {
        0 => { assert_eq!(x,0); x = 1; },
        1 => { assert_eq!(x,1); x = 2; break; },
        _ => { panic!("Should not reach the default case"); },
    });
    assert_eq!(x, 2);
}
```

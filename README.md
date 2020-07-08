[![crates.io](https://img.shields.io/crates/v/web-butter.svg)](https://crates.io/crates/web-butter)
[![Documentation](https://docs.rs/web-butter/badge.svg)](https://docs.rs/web-butter)
[![MIT](https://img.shields.io/crates/l/web-butter.svg)](./LICENSE)

# web-butter

A simple no-cost wrapper for the web_sys crate to get auto-completes from IDE.

[Docs](https://docs.rs/web-butter)

## Install

Add this to your `Cargo.toml` file:

```toml
[dependencies]
web-butter = "1.0.0"
```

## Example

```rust
use web_butter;

fn main() {
  println!("{}", web_butter::add(2, 3));
}
```

## Contributing

Your PRs and suggestions are always welcome.

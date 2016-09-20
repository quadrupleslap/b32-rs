# b32

A simple base-32 encoder/decoder. It only has two functions:

```rust
b32::encode(String) -> String;
b32::decode(String) -> Option<String>;
```

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
b32 = "0.1.0"
```

## License

This tiny piece of code is published under **[the MIT license](LICENSE.txt)**.

## Xeh
[![etc](https://github.com/clearloop/xeh/workflows/xeh/badge.svg)](https://github.com/clearloop/xeh)
[![crate](https://img.shields.io/crates/v/xeh.svg)](https://crates.io/crates/xeh)
[![downloads](https://img.shields.io/crates/d/xeh.svg)](https://crates.io/crates/xeh)
[![LICENSE](https://img.shields.io/crates/l/xeh.svg)](https://choosealicense.com/licenses/mit/)

The Lightest Hex Dependency in Rust

If you REALLY want to import an `hex` dependency to handle `hex` stuffs in rust, 
this is exactly what you are looking for, `no_std`, `no_alloc`.

```rust
/// Encode str or bytes into hex iterator
xeh::encode<T>(src: &T) -> Option<impl From<SliceVec<char>>> 
where
    T: ToHex;
```

## LICENSE

MIT

[![Crates.io](https://img.shields.io/crates/v/proc-concat-bytes.svg)](https://crates.io/crates/proc-concat-bytes)
[![api](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/proc-concat-bytes/0.1.0/proc-concat-bytes/)

# proc-concat-bytes

Concatenates byte literals
## Examples
```rust
use proc_concat_bytes::concat_bytes;
let c_str = std::ffi::CStr::from_bytes_with_nul(&concat_bytes!(b"Hello World!", b'\0')[..]).unwrap();
```

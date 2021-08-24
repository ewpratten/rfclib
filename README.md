# rfclib
[![Crates.io](https://img.shields.io/crates/v/rfclib)](https://crates.io/crates/rfclib) 
[![Docs.rs](https://docs.rs/rfclib/badge.svg)](https://docs.rs/rfclib) 
[![Build](https://github.com/Ewpratten/rfclib/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/rfclib/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/rfclib/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/rfclib/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/rfclib/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/rfclib/actions/workflows/audit.yml)

Rust library for querying IETF RFCs

## Example

```rust
let rfc = rfclib::query_rfc(2549).await.unwrap();
assert_eq!(rfc.name, "rfc2549");
assert_eq!(rfc.title, "IP over Avian Carriers with Quality of Service");
```

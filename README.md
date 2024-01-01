# `ptcrab` 🦀 [![crates.io](https://img.shields.io/crates/v/ptcrab.svg)](https://crates.io/crates/ptcrab) [![docs](https://docs.rs/ptcrab/badge.svg)](https://docs.rs/crate/ptcrab)

A pure-Rust alternative to [the official pxtone library](https://pxtone.org/developer).

## Features

- Support for systems of any endianness
- Support for generic Wasm targets
- I/O using standard [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) & [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) traits
- Ptvoice data manipulation

### Not yet implemented

- Ptnoise data manipulation
- Project data (ptcop/pttune) manipulation
- Audio rendering/playback

## License

[MIT](LICENSE)

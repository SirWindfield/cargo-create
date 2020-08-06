# jen (cargo-create)

[![Maintenance](https://img.shields.io/badge/maintenance-actively%20maintained-brightgreen.svg)](https://github.com/SirWindfield/carg-create)
[![crates.io](https://img.shields.io/crates/v/cargo-create.svg)](https://crates.io/crates/cargo-create)
[![crates.io](https://img.shields.io/crates/d/cargo-create)](https://crates.io/crates/cargo-create)
[![Documentation](https://docs.rs/cargo-create/badge.svg)](https://docs.rs/cargo-create)

![Continuous Integration](https://github.com/SirWindfield/cargo-create/workflows/Continuous%20Integration/badge.svg)

> A CLI for fast project generation based on Tera templates. Also a cargo subcommand :)

## Installation

```text
cargo install cargo-create --locked
```

The crate does install the CLI under two names: `jen` and `cargo-create`, allowing for a nicer Rust-related workflow.

## Examples

An example template repository can be found over at https://github.com/SirWindfield/template-test. For a more complex example take a look at the `template` branch of https://github.com/SirWindfield/zerotask-rust-bin-template/tree/template.

## Note

The public API of the library crate is __NOT__ meant for consumption. It is not stable in any way, may change any time and is not
related to the semantic versioning of this repository. The crate type is a library to allow for multiple binaries to exist in one
single place.

The semantic versioning does only apply to the overall CLI tool, aka the binaries itself and their public API.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

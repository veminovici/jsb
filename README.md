# JSB Rust Workspace

This is a Rust workspace containing the JSB Standard Library.

## Project Structure

```
.
├── Cargo.toml          # Workspace configuration
├── jsb-std/           # JSB Standard Library crate
│   ├── Cargo.toml     # Library configuration
│   └── src/
│       ├── lib.rs      # Library entry point
│       └── prelude.rs  # Prelude module
└── README.md           # This file
```

## Getting Started

### Prerequisites

- Rust toolchain (install via [rustup](https://rustup.rs/))

### Building

To build all crates in the workspace:

```bash
cargo build
```

To build a specific crate:

```bash
cargo build -p jsb-std
```

### Testing

To run tests for all crates:

```bash
cargo test
```

To run tests for a specific crate:

```bash
cargo test -p jsb-std
```

### Documentation

To generate documentation:

```bash
cargo doc --open
```

## Adding New Crates

To add a new crate to the workspace:

1. Create a new directory for your crate
2. Add the crate name to the `members` array in the root `Cargo.toml`
3. Create a `Cargo.toml` and source files for your crate

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

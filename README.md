# add_app

A simple Rust workspace containing two crates: a library crate `adder` and a binary crate `main`.  
This project demonstrates how to structure a Rust workspace and share code between crates.

## Workspace Structure

```
add_app/
│── Cargo.toml        # Workspace manifest
│── .gitignore
│
├── adder/            # Library crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
│
├── main/             # Binary crate
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
```

- `adder` is a library crate that exposes reusable functions, like `add`.
- `main` is a binary crate that depends on `adder` and uses its functions.

## Usage

1. Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/add_app.git
cd add_app
```

2. Build the workspace:

```bash
cargo build
```

3. Run the binary crate `main`:

```bash
cargo run -p main
```

Expected output:

```
Hello, world!
3
```

## Notes

- The `target/` folder is ignored via `.gitignore` since it contains build artifacts.
- This workspace structure allows multiple crates to share code easily.
- Modify `adder/src/lib.rs` to add more reusable functions, which can be used by `main`.

## License

MIT License

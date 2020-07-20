# Rust-API
Example API service with Rust.

[Tutorial]("https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html")

## Create the workspace
1. Create `Cargo.toml` with:
```sh
workspace

members = [
    "main",
    "lib_a"
]
```
2. Go in folder and create the adder binary crate by running `cargo new` (main).
3. Generate a new library crate named `lib-a`:
```sh
cargo new lib_a --lib
```

## Rust Web Framework
### Actix-Web
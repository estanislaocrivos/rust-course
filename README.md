# Rust Fundamentals Course @ [Udemy](https://www.udemy.com/curso-completo-rust)

## Annotations

- For installing Rust you may run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- For creating new Rust applications (binaries) you can use Cargo (Rust's package manager):

```bash
cargo new application-name
```

- For creating new Rust libraries, run Cargo with the `--lib` option:

```bash
cargo new library-name --lib
```

- For checking the project for errors, run:

```bash
cargo check
```

- For building the project, run:

```bash
cargo build
```

Then you may run the executable file inside `target/debug/`. You may also add the `--release` flag to tell the compiler to apply optimizations on the project.

- For building + running the project, run:

```bash
cargo run
```

- For adding crates to our project, you may run:

```bash
cargo add crate-name
```

You can find available crates on [crates.io](https://crates.io).

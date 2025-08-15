# Rust Fundamentals Course Notes and Resources

This repository contains notes and resources from two Udemy courses I took on the Rust programming language:

- [Learn to Code with Rust](https://www.udemy.com/course/learn-to-code-with-rust/)
- [Curso Completo de Rust](https://www.udemy.com/curso-completo-rust/)

The `main.rs` file contains examples on each topic covered in the courses. Additional files can be found in the `src` directory, which include solutions to the exercises provided in the courses.

## Notes

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

- For creating and running tests, you can use the `test` macro:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```

In Rust it is a common convention to place unit tests in the same file as the source code under test, but separated in a module as shown above. For running the tests, you can use the following command:

```bash
cargo test
```

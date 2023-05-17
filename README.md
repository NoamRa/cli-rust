# Install Rust

https://www.rust-lang.org/tools/install

# Book's GitHub Repo

https://github.com/kyclark/command-line-rust

# Command-Line Rust exercises

Following Command-Line Rust by Ken Youens-Clark

## Compile rust code

```console
rustc filename.rs
```

and then run the code

```sh
filename
```

## create new project

```sh
cargo new {project name}
```

to create a project without git repo (useful for these exercises)

```sh
cargo new {project name} --vcs=none
```

## Compile and run with Cargo

In project folder

```sh
cargo run
```

# Test

mark test function with `#[test]`

```rust
#[test]
fn works() {
    assert!(true);
}
```

and run

```sh
cargo test
```

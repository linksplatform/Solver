# Solver
[![Gitpod](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/linksplatform/solver) 

Algorithms to find shortest and simplest possible functions for given input and output data ranges.

## Prerequisites

Usually it is enough to use `rust-toolchain.toml` file for `cargo` configuration. In case you need to install required toolchain manually, you can use that command:

```bash
rustup toolchain install stable && cargo +stable build
```

## Run

```bash
cargo run
```

## Save result to file

```bash
cargo run 2>&1 | tee result.txt
```

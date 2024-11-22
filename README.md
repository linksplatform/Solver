# Solver
[![Gitpod](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/linksplatform/solver) 

Algorithms to find shortest and simplest possible functions for given input and output data ranges.

## Prerequsites

Usually it is enough to use `rust-toolchain.toml` file for `cargo` configuration. In case you need to install required toolchain manually, you can use that command:

```bash
rustup toolchain install nightly-2022-08-22 && cargo +nightly-2022-08-22 build
```

## Run

```bash
cargo run
```

## Save result to file

```bash
cargo run 2>&1 | tee result.txt
```

## Troubleshooting

If you get error:

```
error: package `bumpalo v3.16.0` cannot be built because it requires rustc 1.73.0 or newer, while the currently active rustc version is 1.65.0-nightly
Either upgrade to rustc 1.73.0 or newer, or use
cargo update -p bumpalo@3.16.0 --precise ver
where `ver` is the latest version of `bumpalo` supporting rustc 1.65.0-nightly
```

You might need to execute this command:

```bash
cargo update -p bumpalo@3.16.0 --precise 3.11.1
```
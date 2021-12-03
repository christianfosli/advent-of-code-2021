# Advent of Code 2021

[![ci](https://github.com/christianfosli/advent-of-code-2021/actions/workflows/ci.yaml/badge.svg)](https://github.com/christianfosli/advent-of-code-2021/actions/workflows/ci.yaml)

My solutions for [advent of code](https://adventofcode.com) 2021, using rust.

## Running locally

Ensure you have [rust installed](https://www.rust-lang.org/tools/install).

`cd` into one of the folders for a specific day, then

```bash
cargo test # run tests
cargo run  # run main - find the answer

# or if you want max performance compile a release binary first
cargo build --release
./target/day0x
```

If you prefer using docker you can do something like this
```bash
docker run --rm --user "$(id -u):$(id -g)" -v "$PWD:/usr/src/app" -w "/usr/src/app" rust cargo run
```

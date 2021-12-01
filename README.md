# Advent of Code 2021

[![ci](https://github.com/christianfosli/advent-of-code-2021/actions/workflows/ci.yaml/badge.svg)](https://github.com/christianfosli/advent-of-code-2021/actions/workflows/ci.yaml)

My solutions for [advent of code](https://adventofcode.com) 2021, using rust.

## Running locally

Ensure you have [rust installed](https://www.rust-lang.org/tools/install), then
```bash
cd day0x
cargo test # run tests
cargo run  # run solution for day0x

# or if you want max performance compile a release binary first
cargo build --release
./target/day0x
```

If you prefer using docker you can do something like this
```bash
cd day0x
docker run --rm --user "$(id -u):$(id -g)" -v "$PWD:/usr/src/app" -w "/usr/src/app" rust cargo run
```

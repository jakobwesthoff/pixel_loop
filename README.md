# 🎨 Pixel Loop 🔁

**WORK IN PROGRESS**

## Overview
Pixel Loop is a Rust implementation inspired by the concepts discussed in the [article on fixed time game/update loops by Gaffer on Games](https://gafferongames.com/post/fix_your_timestep/). The current implementation leverages [tao](https://crates.io/crates/tao) and [pixels](https://crates.io/crates/pixels) for window initialization and drawing capabilities. The intention is to further generalize this implementation in the future.

## Motivation
The idea behind Pixel Loop resonated with me as I have often faced challenges with timing aspects while working on animations from scratch. This project serves as a practical exploration of fixed time game/update loops and lays the groundwork for future experiments and projects.

## Build Instructions
To build Pixel Loop using Cargo, execute the following command:

```shell
cargo build --release
```

## Running the Application
Once built, the executable can be located at `target/release/pixelloop`. To run the application, simply execute:
```
target/release/pixelloop
```

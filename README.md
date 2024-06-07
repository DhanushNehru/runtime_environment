# runtime-environment

A Rust library to check the operating system at runtime.

## Installation

In your `Cargo.toml`, add the following line under the `[dependencies]` section:

```rust
runtime-environment = "0.1.0"
```

## Usage
This is an example usage someone might do:

```rust
use runtime-environment::{is_mac_os, is_windows, is_linux};

fn main() {
    if is_mac_os() {
        println!("Running on macOS");
    } else if is_windows() {
        println!("Running on Windows");
    } else if is_linux() {
        println!("Running on Linux");
    } else {
        println!("Running on an unknown OS");
    }
}
```
## Functions

is_mac_os() -> bool

Returns true if the operating system is macOS.

is_windows() -> bool

Returns true if the operating system is Windows.

is_linux() -> bool

Returns true if the operating system is Linux.

## Testing
To run the tests, use the following command:

```rust
cargo test
```

## License

This project is licensed under the MIT License.

Replace `"0.1.0"` with the actual version number of your package when you publish it. 

This `README.md` file provides a clear overview of the library, installation instructions, example usage, and information on the available functions.

## Gitpod

In the cloud-free development runtime-environment where you can directly start coding.

You can use Gitpod in the cloud  [![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/DhanushNehru/runtime_environment/)

----

Feel free to update the README.md or raise issues if any to enhance the project

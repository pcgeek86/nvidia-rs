# NVIDIA CLI

This is a command line tool to interface with NVIDIA GPUs.
This tool has been developed in native Rust code. 🦀
We rely on the `nvml-wrapper` crate, developed by [@cldfire](https://github.com/cldfire/).

![screenshot](https://github.com/pcgeek86/nvidia-rs/blob/master/assets/screenshot.png?raw=true)

# Installation 

Ensure that the [Rust toolchain](https://rustup.rs) is installed on your system.

```bash
cargo install nvidia
```

# Usage

Run this command:

```
nvidia
```

# Uninstallation

You can remove the NVIDIA CLI tool with the following command:

```
cargo uninstall nvidia
```

# License

MIT

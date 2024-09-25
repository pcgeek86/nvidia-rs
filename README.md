# NVIDIA CLI

This is a command line tool to interface with NVIDIA GPUs.
This tool has been developed in native Rust code. 🦀
We rely on the `nvml-wrapper` crate, developed by [@cldfire](https://github.com/cldfire/).
For the terminal user interface, we use the popular [ratatui](https://github.com/ratatui/ratatui) crate.

![screenshot](https://github.com/pcgeek86/nvidia-rs/blob/master/assets/screenshot02.png?raw=true)

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

The program refreshes statistics every 200 milliseconds.
At the moment, this threshold is not configurable.
To exit the program, press the `q` key on your keyboard.

# Uninstallation

You can remove the NVIDIA CLI tool with the following command:

```
cargo uninstall nvidia
```

# Possible Issues

* This application only looks at the first NVIDIA GPU in your system. Multiple GPUs not currently supported
* If you don't have the NVIDIA GeForce driver installed, this utility won't work
* Only tested this application on Windows 11

# License

MIT

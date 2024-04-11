# Vexide Template

[![Build status](https://github.com/vexide/vexide-template/actions/workflows/build.yml/badge.svg)](https://github.com/vexide/vexide-template/actions/workflows/build.yml)

> Ready-to-use template for developing VEX V5 robots in Rust.

Seasoned Vexide user? Delete README.md and update Cargo.toml as needed.

## Table of Contents

- [Vexide Template](#vexide-template)
  - [Table of Contents](#table-of-contents)
  - [Using This Template](#using-this-template)
  - [Getting Started (Windows)](#getting-started-windows)
  - [Getting Started (macOS)](#getting-started-macos)
  - [Getting Started (Debian/Ubuntu Linux)](#getting-started-debianubuntu-linux)
  - [Getting Started (Fedora Linux)](#getting-started-fedora-linux)
  - [Development](#development)
    - [Compiling and uploading to a VEX V5 robot](#compiling-and-uploading-to-a-vex-v5-robot)
    - [Using smart editing features](#using-smart-editing-features)
  - [Troubleshooting](#troubleshooting)

## Using This Template

To start a project using this template, click the "Use this template" button in the upper right corner of the GitHub repository. Choose an appropriate name and clone the new repository using Git. Finally, update the package name in `Cargo.toml`:

```toml
[package]
name = "my-vex-robot"
version = "0.1.0"
edition = "2021"
```

## Getting Started (Windows)

Install Rust by following the instructions on <https://rustup.rs/>. Configure your installation when prompted, installing a **nightly** toolchain and keeping other default values.

Run the following commands in Powershell to set up your PC for development on Windows.

- Install Python 3.9 and the Embedded ARM Toolchain. You may be prompted for Administrator access.

  ```console
  winget install -s msstore "Python 3.9"
  winget install Arm.GnuArmEmbeddedToolchain
  ```

- Close and reopen the terminal, and finish installing Vexide:

  ```console
  pip3.9 install --user pros-cli
  rustup component add rust-src
  cargo install cargo-pros
  ```

## Getting Started (macOS)

Run the following terminal commands to set up your Mac for development.

- Install Homebrew, a package manager for macOS.

  ```console
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/  install/HEAD/install.sh)"
  ```

  - Under the header "Next Steps", Homebrew may prompt you to run commands to complete the installation.

- Install Rustup, the Embedded ARM Toolchain, and PROS:

  ```console
  brew install rustup osx-cross/arm/arm-gcc-bin purduesigbots/pros/pros-cli
  ```

- Use Rustup to setup Rust.

  ```console
  rustup-init -y --default-toolchain nightly
  ```

- Close and reopen the terminal, and finish installing Vexide:

  ```console
  rustup component add rust-src
  cargo install cargo-pros
  ```

## Getting Started (Debian/Ubuntu Linux)

Run the following terminal commands to set up your PC for development on Debian or Ubuntu.

- Install Rustup.

  ```console
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Install Python 3.9 and PROS.

  ```console
  sudo add-apt-repository ppa:deadsnakes/ppa
  sudo apt update
  sudo apt install python3.9 python3-pip python3.9-distutils

  python3.9 -m pip install --user pros
  ```

- Close and reopen the terminal, and finish installing Vexide:

  ```console
  rustup default nightly
  rustup component add rust-src --toolchain nightly
  cargo install cargo-pros
  ```

## Getting Started (Fedora Linux)

Run the following terminal commands to set up your PC for development on Fedora.

- Install Rustup, the Embedded ARM Toolchain, PROS, and GCC:

  ```console
  sudo dnf install rustup python3-pip arm-none-eabi-gcc-cs gcc
  rustup-init -y --default-toolchain nightly
  pip install --user pros-cli
  ```

- Close and reopen the terminal, and finish installing Vexide:

  ```console
  rustup component add rust-src
  cargo install cargo-pros
  ```

## Development

### Compiling and uploading to a VEX V5 robot

Use the Cargo PROS terminal utility to build this pros-rs project.

```console
cargo pros build
```

The separate `pros` command is used to upload. Plug in your powered VEX robot brain via USB and run the following command to upload to program slot 1. If you changed the package name when setting up this template, you may need to adapt the name of the `.bin` file.

```console
pros upload --target v5 --slot 1 ./target/armv7a-vexos-eabi/debug/vexide-template.bin
```

<!--
### Debugging in the pros-rs simulator

If you have PROS Simulator installed, you can use it to run this project without real VEX hardware for debugging and development purposes. Start by adding the WebAssembly Rust target:

```console
rustup target add wasm32-unknown-unknown
```

Build the project for the simulator by running:

```console
cargo pros build -s
```

Then open this project in PROS Simulator to run and debug the robot code.
-->
### Using smart editing features

Developers using Visual Studio Code with the rust-analyzer extension have access to smart editing features like Intellisense and code analysis. By default, rust-analyzer will check the project for errors when it is saved.

## Troubleshooting

- If you get the error `TypeError: <flag 'BrainFlags'> has no members defined` when using the third party `pros upload` command, you need to downgrade Python to v3.9

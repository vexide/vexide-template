# Vexide Template

[![Build status](https://github.com/vexide/vexide-template/actions/workflows/build.yml/badge.svg)](https://github.com/vexide/vexide-template/actions/workflows/build.yml)

> Ready-to-use template for developing VEX V5 robots in Rust.

Seasoned vexide user? Delete README.md and update Cargo.toml as needed.

## Table of Contents

- [Vexide Template](#vexide-template)
  - [Table of Contents](#table-of-contents)
  - [Using This Template](#using-this-template)
  - [Getting Started (Windows)](#getting-started-windows)
  - [Getting Started (macOS)](#getting-started-macos)
  - [Getting Started (NixOS)](#getting-started-nixos)
  - [Getting Started (Debian/Ubuntu Linux)](#getting-started-debianubuntu-linux)
  - [Getting Started (Fedora Linux)](#getting-started-fedora-linux)
  - [Learn](#learn)
  - [Development](#development)
    - [Compiling and uploading to a VEX V5 robot](#compiling-and-uploading-to-a-vex-v5-robot)
    - [Viewing program output](#viewing-program-output)

## Using This Template

To start a project using this template, click the "Use this template" button in the upper right corner of the GitHub repository. Choose an appropriate name and clone the new repository using Git. Finally, update the package name in `Cargo.toml`:

```toml
[package]
name = "my-vex-robot"
version = "0.1.0"
edition = "2021"
```

## Getting Started (Windows)

Install Rust by following the instructions on <https://rustup.rs/>.

Run the following commands in Powershell to set up your PC for development on Windows.


- Install cargo-v5:

  ```console
  rustup default nightly
  rustup component add rust-src
  cargo install cargo-v5
  ```

## Getting Started (macOS)

Run the following terminal commands to set up your Mac for development.

- Install Homebrew, a package manager for macOS:

  ```console
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  ```

  - Under the header "Next Steps", Homebrew may prompt you to run commands to complete the installation.

- Install Rust and PROS:

  ```console
  brew install rustup python@3.10
  rustup-init -y --default-toolchain nightly
  pip3.10 install pros-cli
  ```

- Close and reopen the terminal, and finish installing vexide:

  ```console
  rustup component add rust-src llvm-tools-preview
  cargo install cargo-pros cargo-binutils
  ```

## Getting Started (NixOS)

The Nix flake includes a devshell with every tool you need for building and uploading vexide projects.

There is a `.envrc` file for Nix + Direnv users.

## Getting Started (Debian/Ubuntu Linux)

Run the following terminal commands to set up your PC for development on Debian or Ubuntu.

- Install Rust:

  ```console
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Install cargo-v5:

  ```console
  rustup default nightly
  rustup component add rust-src
  cargo install cargo-v5
  ```

## Getting Started (Fedora Linux)

Run the following terminal commands to set up your PC for development on Fedora.

- Install Rust:

  ```console
  sudo dnf install rustup
  rustup-init -y --default-toolchain nightly
  ```

- Close and reopen the terminal, and finish installing vexide:

  ```console
  rustup component add rust-src
  cargo install cargo-v5
  ```

## Learn

[Check out the documentation](https://vexide.dev/docs/) on the official vexide website for walkthrough-style guides and other helpful learning resources!

An [API reference](https://docs.rs/vexide) is also provided by docs.rs.

## Development

### Compiling and uploading to a VEX V5 robot

Use the cargo-pros terminal utility to build and upload this vexide project.

```console
cargo v5 build
```

Use a USB cable to connect to your robot brain or to your controller before using the `upload` subcommand to build and upload the project. Make sure to specify a program slot.

```console
cargo v5 upload
```

### Viewing program output

You can view panic messages and calls to `println!()` using the PROS terminal.
Use a USB cable to connect to your robot brain or controller, then start the terminal:

```console
cargo v5 terminal
```

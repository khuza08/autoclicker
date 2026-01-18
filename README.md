# Rust AutoClicker for Linux

A simple autoclicker application built with Rust for Linux systems.

## Prerequisites

Before building and running this application, you need to install the following system dependencies:

### On Arch-based distributions (Arch, Manjaro, CachyOS):
```bash
sudo pacman -S xdotool libxtst libxinerama libxcursor libxrandr libxss
```

### On Ubuntu/Debian-based distributions:
```bash
sudo apt-get install xdotool libxtst-dev libxinerama-dev libxcursor-dev libxrandr-dev libxss-dev
```

### On Fedora/RHEL-based distributions:
```bash
sudo dnf install xdotool libXtst-devel libXinerama-devel libXcursor-devel libXrandr-devel libXScrnSaver-devel
```

## Building

To build the application, run:

```bash
cargo build --release
```

## Running

To run the application in development mode:

```bash
cargo run
```

## Features

- Configurable click interval
- Support for left, right, and middle mouse buttons
- Start/stop functionality
- Command-line interface

## Usage

The application provides a command-line interface:
- Press `s` to start autoclicking
- Press `t` to stop autoclicking
- Press `q` to quit the application

## License

This project is licensed under the MIT License - see the LICENSE file for details.

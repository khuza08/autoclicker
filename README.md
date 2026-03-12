# Rust AutoClicker

A simple and robust autoclicker application built with Rust. Supports both a modern GUI and a lightweight CLI.

## Prerequisites

Before building and running this application, you need to install the following system dependencies (required by `enigo` and `egui`):

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

### GUI Mode (Default)
To run the graphical interface:
```bash
cargo run
```

### CLI Mode
To run in the terminal:
```bash
cargo run -- --cli
```

## Features

- **Dual Modes**: Modern GUI or lightweight CLI.
- **Configurable Interval**: Adjust delay from 10ms to 5000ms.
- **Click Types**: Support for left, right, and middle mouse buttons.
- **Robust Error Handling**: Gracefully handles mouse initialization failures.
- **Optimized Performance**: Low CPU usage through intelligent repainting.

## CLI Usage
When running in `--cli` mode:
- `s`: Start autoclicking
- `t`: Stop autoclicking
- `+`: Increase delay (by 100ms)
- `-`: Decrease delay (by 100ms)
- `q`: Quit the application

## License

This project is licensed under the MIT License - see the LICENSE file for details.

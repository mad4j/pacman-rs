# pacman-rs

Pac-Man arcade emulator written in Rust, based on MAME driver architecture.

## Overview

This project aims to accurately emulate the original Pac-Man arcade hardware using a modular architecture inspired by the MAME (Multiple Arcade Machine Emulator) driver structure.

## Architecture

The emulator is organized into the following software components, mirroring the actual Pac-Man arcade hardware:

### Core Components

- **CPU Module** (`src/cpu/`)
  - Z80 CPU emulation @ 3.072 MHz
  - Interrupt handling (60Hz vertical blank)
  - Instruction execution and timing

- **Memory Module** (`src/memory/`)
  - Memory bus implementation
  - ROM management (16KB program ROM)
  - RAM regions (Video RAM, Color RAM, System RAM)
  - Memory-mapped I/O
  - Complete memory map:
    - `0x0000-0x3FFF`: Game ROM (16KB)
    - `0x4000-0x43FF`: Video RAM (1KB)
    - `0x4400-0x47FF`: Color RAM (1KB)
    - `0x4800-0x4FEF`: System RAM (2KB)
    - `0x4FF0-0x4FFF`: Sprite data (16 bytes)
    - `0x5000-0x50FF`: I/O ports

- **Video Module** (`src/video/`)
  - Tile-based rendering system (28x36 tiles, 8x8 pixels each)
  - Sprite rendering (up to 6 sprites)
  - Display resolution: 224x288 pixels
  - Palette management

- **Sound Module** (`src/sound/`)
  - Namco WSG (Waveform Sound Generator) emulation
  - 3-voice sound synthesis
  - Audio mixing and output

- **Input Module** (`src/input/`)
  - Joystick input (4-direction)
  - Button handling (Start 1P/2P, Coin)
  - Input state management

## Project Structure

```
pacman-rs/
├── Cargo.toml          # Rust project configuration
├── README.md           # This file
├── roms/               # ROM files directory (not in repository)
│   └── README.md       # ROM requirements and legal notice
└── src/
    ├── lib.rs          # Library entry point
    ├── main.rs         # Binary entry point
    ├── cpu/            # CPU emulation
    │   └── mod.rs
    ├── memory/         # Memory subsystem
    │   └── mod.rs
    ├── video/          # Video hardware
    │   └── mod.rs
    ├── sound/          # Sound hardware
    │   └── mod.rs
    └── input/          # Input system
        └── mod.rs
```

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release -- roms/pacman.rom
```

## ROM Files

ROM files must be placed in the `roms/` directory. See `roms/README.md` for details on required files and legal information.

**Note**: ROM files are copyrighted by Namco/Bandai Namco and are not included in this repository.

## Development Status

This is the initial project structure. The following components are outlined but not yet fully implemented:

- [x] Project structure and module organization
- [ ] Z80 CPU instruction set implementation
- [ ] Complete memory mapping
- [ ] Tile and sprite rendering
- [ ] Sound generation
- [ ] Input handling
- [ ] Main emulation loop
- [ ] Display output (SDL2, Pixels, or similar)
- [ ] Audio output

## References

- MAME Pac-Man driver source code
- Pac-Man hardware specifications
- Z80 CPU documentation

## License

MIT License - See LICENSE file for details.

**Note**: This emulator is for educational purposes. Original Pac-Man ROM files are copyrighted by Namco/Bandai Namco.

# ROMs Directory

This directory is intended for the Pac-Man ROM archive.

## Required File

The emulator expects a ZIP archive named **`pacman.zip`** containing the following ROM files:

### Program ROMs (16KB total)
The ZIP archive must contain these 4 program ROM files (4KB each):
- `pacman.6e` - 4KB program ROM
- `pacman.6f` - 4KB program ROM
- `pacman.6h` - 4KB program ROM
- `pacman.6j` - 4KB program ROM

### Additional ROM Files (for future implementation)
The ZIP archive may also contain:
- `pacman.5e` - 4KB character graphics ROM
- `pacman.5f` - 4KB sprite graphics ROM
- `82s123.7f` - 32 bytes color palette PROM
- `82s126.4a` - 256 bytes color lookup PROM
- `82s126.1m` - 256 bytes sprite color lookup PROM

## Legal Notice

**Important**: ROM files are copyrighted by Namco/Bandai Namco. 

You must own the original Pac-Man arcade board or have legal rights to use these ROM files. This emulator is for educational and preservation purposes only.

## File Organization

Place your `pacman.zip` archive directly in this directory:

```
roms/
└── pacman.zip
    ├── pacman.6e
    ├── pacman.6f
    ├── pacman.6h
    ├── pacman.6j
    ├── pacman.5e
    ├── pacman.5f
    ├── 82s123.7f
    ├── 82s126.4a
    └── 82s126.1m
```

## Usage

Run the emulator with:
```bash
cargo run --release -- roms/pacman.zip
```

## Note

This directory is excluded from version control (via `.gitignore`) to prevent accidental distribution of copyrighted ROM files.

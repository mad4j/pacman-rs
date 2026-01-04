# ROMs Directory

This directory is intended for Pac-Man arcade ROM files.

## Required ROM Files

The Pac-Man arcade machine uses the following ROM files:

### Program ROMs (16KB total)
- `pacman.6e` - 4KB program ROM
- `pacman.6f` - 4KB program ROM
- `pacman.6h` - 4KB program ROM
- `pacman.6j` - 4KB program ROM

### Character/Tile Graphics ROM
- `pacman.5e` - 4KB character graphics

### Sprite Graphics ROM
- `pacman.5f` - 4KB sprite graphics

### Color Palette PROMs
- `82s123.7f` - 32 bytes color palette
- `82s126.4a` - 256 bytes color lookup
- `82s126.1m` - 256 bytes sprite color lookup

## Legal Notice

**Important**: ROM files are copyrighted by Namco/Bandai Namco. 

You must own the original Pac-Man arcade board or have legal rights to use these ROM files. This emulator is for educational and preservation purposes only.

## File Organization

Place your ROM files directly in this directory:

```
roms/
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

## Note

This directory is excluded from version control (via `.gitignore`) to prevent accidental distribution of copyrighted ROM files.

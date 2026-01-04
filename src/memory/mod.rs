//! Memory Subsystem Module
//! 
//! Implements the Pac-Man memory map:
//! - 0x0000-0x3FFF: 16KB Game ROM (program code)
//! - 0x4000-0x43FF: 1KB Video RAM (character tiles)
//! - 0x4400-0x47FF: 1KB Color RAM (tile attributes)
//! - 0x4800-0x4FEF: 2KB System RAM
//! - 0x4FF0-0x4FFF: Sprite data
//! - 0x5000-0x50FF: Memory-mapped I/O

use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

const ROM_SIZE: usize = 0x4000;      // 16KB
const VRAM_SIZE: usize = 0x0400;     // 1KB
const COLOR_RAM_SIZE: usize = 0x0400; // 1KB
const RAM_SIZE: usize = 0x07F0;      // 2032 bytes (0x4800-0x4FEF)
const SPRITE_SIZE: usize = 0x0010;   // 16 bytes

/// Memory bus for Pac-Man
/// 
/// Manages all memory regions and provides read/write access
pub struct MemoryBus {
    rom: Vec<u8>,           // Game ROM (0x0000-0x3FFF)
    vram: Vec<u8>,          // Video RAM (0x4000-0x43FF)
    color_ram: Vec<u8>,     // Color RAM (0x4400-0x47FF)
    ram: Vec<u8>,           // System RAM (0x4800-0x4FEF)
    sprite_data: Vec<u8>,   // Sprite data (0x4FF0-0x4FFF)
}

impl MemoryBus {
    /// Creates a new memory bus with initialized regions
    pub fn new() -> Self {
        Self {
            rom: vec![0; ROM_SIZE],
            vram: vec![0; VRAM_SIZE],
            color_ram: vec![0; COLOR_RAM_SIZE],
            ram: vec![0; RAM_SIZE],
            sprite_data: vec![0; SPRITE_SIZE],
        }
    }

    /// Loads ROM data from a ZIP archive
    /// 
    /// The ZIP archive should contain the program ROM files:
    /// - pacman.6e, pacman.6f, pacman.6h, pacman.6j (4KB each)
    /// These are concatenated to form the 16KB program ROM
    pub fn load_rom(&mut self, zip_path: &str) -> Result<(), String> {
        let path = Path::new(zip_path);
        
        // Open ZIP archive
        let file = File::open(path)
            .map_err(|e| format!("Failed to open ZIP file: {}", e))?;
        
        let mut archive = ZipArchive::new(file)
            .map_err(|e| format!("Failed to read ZIP archive: {}", e))?;
        
        // ROM file names in order (4KB each = 16KB total)
        let rom_files = ["pacman.6e", "pacman.6f", "pacman.6h", "pacman.6j"];
        let mut rom_offset = 0;
        
        for rom_name in &rom_files {
            match archive.by_name(rom_name) {
                Ok(mut file) => {
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)
                        .map_err(|e| format!("Failed to read {} from ZIP: {}", rom_name, e))?;
                    
                    // Each ROM should be 4KB (0x1000 bytes)
                    if buffer.len() != 0x1000 {
                        return Err(format!("{} should be 4KB, found {} bytes", rom_name, buffer.len()));
                    }
                    
                    // Copy to ROM at the correct offset
                    if rom_offset + buffer.len() <= ROM_SIZE {
                        self.rom[rom_offset..rom_offset + buffer.len()].copy_from_slice(&buffer);
                        rom_offset += buffer.len();
                    } else {
                        return Err(format!("ROM data exceeds {} bytes", ROM_SIZE));
                    }
                }
                Err(_) => {
                    return Err(format!("ROM file {} not found in ZIP archive", rom_name));
                }
            }
        }
        
        Ok(())
    }

    /// Reads a byte from the specified address
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom[address as usize],
            0x4000..=0x43FF => self.vram[(address - 0x4000) as usize],
            0x4400..=0x47FF => self.color_ram[(address - 0x4400) as usize],
            0x4800..=0x4FEF => self.ram[(address - 0x4800) as usize],
            0x4FF0..=0x4FFF => self.sprite_data[(address - 0x4FF0) as usize],
            0x5000..=0x50FF => {
                // TODO: Implement I/O read
                0
            }
            _ => 0,
        }
    }

    /// Writes a byte to the specified address
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => {
                // ROM is read-only
            }
            0x4000..=0x43FF => self.vram[(address - 0x4000) as usize] = value,
            0x4400..=0x47FF => self.color_ram[(address - 0x4400) as usize] = value,
            0x4800..=0x4FEF => self.ram[(address - 0x4800) as usize] = value,
            0x4FF0..=0x4FFF => self.sprite_data[(address - 0x4FF0) as usize] = value,
            0x5000..=0x50FF => {
                // TODO: Implement I/O write
            }
            _ => {}
        }
    }

    /// Resets all writable memory to zero
    pub fn reset(&mut self) {
        self.vram.fill(0);
        self.color_ram.fill(0);
        self.ram.fill(0);
        self.sprite_data.fill(0);
    }

    /// Returns a reference to video RAM
    pub fn vram(&self) -> &[u8] {
        &self.vram
    }

    /// Returns a reference to color RAM
    pub fn color_ram(&self) -> &[u8] {
        &self.color_ram
    }

    /// Returns a reference to sprite data
    pub fn sprite_data(&self) -> &[u8] {
        &self.sprite_data
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self::new()
    }
}

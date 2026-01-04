//! Video System Module
//! 
//! Implements Pac-Man video hardware:
//! - Tile-based background (28x36 tiles, 8x8 pixels each)
//! - Sprite layer (up to 6 sprites)
//! - Display resolution: 224x288 pixels
//! - Palette-based color system

use crate::memory::MemoryBus;

const SCREEN_WIDTH: usize = 224;
const SCREEN_HEIGHT: usize = 288;
const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 8;
const TILES_X: usize = 28;
const TILES_Y: usize = 36;
const MAX_SPRITES: usize = 6;

/// Video system for Pac-Man
/// 
/// Manages tile-based rendering and sprite display
pub struct VideoSystem {
    framebuffer: Vec<u8>,
    palette: Vec<u32>,
}

impl VideoSystem {
    /// Creates a new video system
    pub fn new() -> Self {
        Self {
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 4], // RGBA
            palette: Self::init_palette(),
        }
    }

    /// Initializes the color palette
    /// TODO: Load actual Pac-Man palette from color PROM data
    fn init_palette() -> Vec<u32> {
        vec![0xFF000000; 256] // Placeholder palette
    }

    /// Renders a complete frame
    pub fn render_frame(&mut self, memory: &MemoryBus) {
        self.render_background(memory);
        self.render_sprites(memory);
    }

    /// Renders the tile-based background
    fn render_background(&mut self, memory: &MemoryBus) {
        let vram = memory.vram();
        let color_ram = memory.color_ram();

        for tile_y in 0..TILES_Y {
            for tile_x in 0..TILES_X {
                let tile_index = tile_y * TILES_X + tile_x;
                let _tile_code = vram[tile_index];
                let _tile_color = color_ram[tile_index];
                
                // TODO: Implement tile rendering using character ROM data
            }
        }
    }

    /// Renders sprites
    fn render_sprites(&mut self, memory: &MemoryBus) {
        let sprite_data = memory.sprite_data();

        for sprite_idx in 0..MAX_SPRITES {
            let offset = sprite_idx * 2;
            if offset + 1 < sprite_data.len() {
                let _sprite_x = sprite_data[offset];
                let _sprite_y = sprite_data[offset + 1];
                
                // TODO: Implement sprite rendering using sprite ROM data
            }
        }
    }

    /// Resets the video system
    pub fn reset(&mut self) {
        self.framebuffer.fill(0);
    }

    /// Returns the current framebuffer
    pub fn framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }
}

impl Default for VideoSystem {
    fn default() -> Self {
        Self::new()
    }
}

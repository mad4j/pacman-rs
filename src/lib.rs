//! Pac-Man Emulator Library
//! 
//! Based on MAME Pac-Man driver architecture, this library provides
//! the core emulation components for the Pac-Man arcade machine.

pub mod cpu;
pub mod memory;
pub mod video;
pub mod sound;
pub mod input;

/// Pac-Man emulator main structure
/// 
/// Coordinates all hardware components of the Pac-Man arcade machine:
/// - Z80 CPU @ 3.072 MHz
/// - Memory subsystem (ROM, RAM, Video RAM, Color RAM)
/// - Video hardware (tile-based rendering, sprites)
/// - Sound hardware (Namco WSG 3-voice sound)
/// - Input system (joystick, buttons)
pub struct PacmanEmulator {
    cpu: cpu::Z80,
    memory: memory::MemoryBus,
    video: video::VideoSystem,
    sound: sound::SoundSystem,
    input: input::InputSystem,
}

impl PacmanEmulator {
    /// Creates a new Pac-Man emulator instance
    pub fn new() -> Self {
        Self {
            cpu: cpu::Z80::new(),
            memory: memory::MemoryBus::new(),
            video: video::VideoSystem::new(),
            sound: sound::SoundSystem::new(),
            input: input::InputSystem::new(),
        }
    }

    /// Loads ROM data from the specified path
    pub fn load_rom(&mut self, rom_path: &str) -> Result<(), String> {
        self.memory.load_rom(rom_path)
    }

    /// Runs a single frame of emulation (1/60th of a second)
    pub fn run_frame(&mut self) {
        // Run CPU cycles for one frame (~51200 cycles at 3.072 MHz / 60 Hz)
        let cycles_per_frame = 51200;
        self.cpu.run_cycles(&mut self.memory, cycles_per_frame);
        
        // Update video system
        self.video.render_frame(&self.memory);
        
        // Update sound system
        self.sound.update(&self.memory);
    }

    /// Resets the emulator to initial state
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.reset();
        self.video.reset();
        self.sound.reset();
        self.input.reset();
    }
}

impl Default for PacmanEmulator {
    fn default() -> Self {
        Self::new()
    }
}

//! Pac-Man Emulator
//! 
//! Main entry point for the Pac-Man arcade emulator.

use pacman_rs::PacmanEmulator;
use std::env;
use std::path::Path;

fn main() {
    println!("Pac-Man Emulator v0.1.0");
    println!("Based on MAME driver architecture");
    println!();

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }

    let zip_path = &args[1];
    
    // Check if ZIP file exists
    if !Path::new(zip_path).exists() {
        eprintln!("Error: ZIP file not found: {}", zip_path);
        eprintln!();
        eprintln!("Please place pacman.zip in the 'roms/' directory.");
        return;
    }

    // Create and initialize emulator
    let mut emulator = PacmanEmulator::new();
    
    // Load ROM from ZIP
    match emulator.load_rom(zip_path) {
        Ok(_) => {
            println!("ROM loaded successfully from: {}", zip_path);
            println!();
            println!("Emulator initialized with the following components:");
            println!("  - Z80 CPU @ 3.072 MHz");
            println!("  - Memory subsystem (16KB ROM + RAM)");
            println!("  - Video system (224x288 resolution)");
            println!("  - Sound system (Namco WSG 3-voice)");
            println!("  - Input system (joystick + buttons)");
            println!();
            println!("Note: Full emulation loop not yet implemented.");
            println!("This is the initial project structure.");
        }
        Err(e) => {
            eprintln!("Error loading ROM: {}", e);
            return;
        }
    }

    // TODO: Main emulation loop
    // loop {
    //     emulator.run_frame();
    //     // Handle input
    //     // Render video
    //     // Play audio
    // }
}

fn print_usage(program_name: &str) {
    println!("Usage: {} <zip_file>", program_name);
    println!();
    println!("Example:");
    println!("  {} roms/pacman.zip", program_name);
    println!();
    println!("The ZIP archive should contain Pac-Man ROM files.");
}

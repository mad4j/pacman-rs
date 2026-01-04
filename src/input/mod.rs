//! Input System Module
//! 
//! Handles Pac-Man arcade controls:
//! - Joystick (4-direction)
//! - Start buttons (1P, 2P)
//! - Coin slots

/// Input state for Pac-Man controls
#[derive(Debug, Clone, Copy)]
pub struct InputState {
    // Joystick directions
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    
    // Buttons
    pub start_1p: bool,
    pub start_2p: bool,
    pub coin: bool,
}

impl InputState {
    /// Creates a new input state with all inputs cleared
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            start_1p: false,
            start_2p: false,
            coin: false,
        }
    }
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

/// Input system for Pac-Man
/// 
/// Manages input state and provides interface for reading controls
pub struct InputSystem {
    current_state: InputState,
    previous_state: InputState,
}

impl InputSystem {
    /// Creates a new input system
    pub fn new() -> Self {
        Self {
            current_state: InputState::new(),
            previous_state: InputState::new(),
        }
    }

    /// Updates input state
    pub fn update(&mut self, new_state: InputState) {
        self.previous_state = self.current_state;
        self.current_state = new_state;
    }

    /// Returns the current input state
    pub fn state(&self) -> &InputState {
        &self.current_state
    }

    /// Checks if a button was just pressed (not held)
    pub fn pressed(&self, button: InputButton) -> bool {
        let current = self.get_button_state(&self.current_state, button);
        let previous = self.get_button_state(&self.previous_state, button);
        current && !previous
    }

    /// Gets the state of a specific button
    fn get_button_state(&self, state: &InputState, button: InputButton) -> bool {
        match button {
            InputButton::Up => state.up,
            InputButton::Down => state.down,
            InputButton::Left => state.left,
            InputButton::Right => state.right,
            InputButton::Start1P => state.start_1p,
            InputButton::Start2P => state.start_2p,
            InputButton::Coin => state.coin,
        }
    }

    /// Resets the input system
    pub fn reset(&mut self) {
        self.current_state = InputState::new();
        self.previous_state = InputState::new();
    }
}

impl Default for InputSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Enumeration of input buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputButton {
    Up,
    Down,
    Left,
    Right,
    Start1P,
    Start2P,
    Coin,
}

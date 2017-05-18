//! User LEDs

/// pin mode, input or output
pub enum Mode {
    /// input mode
    INPUT, 
    /// output mode
    OUTPUT}

/// Represents a pin
pub trait Pin {
    /// Turns off the Pin
    fn off(&self);

    /// Turns on the Pin
    fn on(&self);
}

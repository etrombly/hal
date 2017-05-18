//! GPIO pins

/// Represents a pin
pub trait Pin {
    /// Turns off the Pin
    fn off(&self);

    /// Turns on the Pin
    fn on(&self);

    /// Return the pin state
    fn digitalRead(&self) -> State;
}

/// pin mode, input or output
pub enum Mode {
    /// input mode
    INPUT, 
    /// output mode
    OUTPUT}

/// pin state, high or low
pub enum State {
    /// high state
    HIGH,
    /// low state
    LOW,
}
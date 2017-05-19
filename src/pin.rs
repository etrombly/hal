//! GPIO pins

/// Represents a pin
pub trait Pin {
    /// Turns off the Pin
    fn off(&self);

    /// Turns on the Pin
    fn on(&self);

    /// Return the pin state
    fn digital_write(&self, state: &State){
        match *state{
            State::HIGH => self.on(),
            State::LOW => self.off(),
        }
    }

    /// Return the pin state
    fn digital_read(&self) -> State;
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
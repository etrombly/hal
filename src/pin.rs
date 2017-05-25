//! GPIO pins

/// Represents a pin
pub trait Pin<T> {
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

    /// Return analog value
    fn analog_read(&self) -> T;

    /// pwm write
    fn analog_write(&self, duty_cycle: u8);
}

/// pin mode, input or output
pub enum Mode {
    /// input
    INPUT, 
    /// analog input
    ANALOG_INPUT,
    /// output
    OUTPUT,
    /// analog output
    ANALOG_OUTPUT,}

/// pin state, high or low
pub enum State {
    /// high state
    HIGH,
    /// low state
    LOW,
}
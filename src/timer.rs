//! Periodic timer

/// General use timer
pub trait Timer {
    /// Resumes the timer count
    fn resume(&self);
    /// Pauses the timer
    fn pause(&self);
}

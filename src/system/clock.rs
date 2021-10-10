use crate::{ffi::system as ffi, system::Time};

pub use ffi::sfClock as Clock;

impl Clock {
    /// Creates a new Clock and starts it automatically.
    #[must_use]
    pub fn start() -> Clock {
        unsafe { ffi::sfClock_create() }
    }

    /// Gets the elapsed time.
    ///
    /// This function returns the time elapsed since the last call to [`restart`]
    /// (or the construction of the instance if [`restart`] has not been called).
    ///
    /// [`restart`]: Clock::restart
    #[must_use]
    pub fn elapsed_time(&self) -> Time {
        unsafe { ffi::sfClock_getElapsedTime(self) }
    }

    /// Restarts the clock.
    ///
    /// This function puts the time counter back to zero.
    /// It also returns the time elapsed since the clock was started.
    pub fn restart(&mut self) -> Time {
        unsafe { ffi::sfClock_restart(self) }
    }
}

impl Default for Clock {
    /// Equivalent to `Clock::start()`.
    fn default() -> Self {
        Clock::start()
    }
}

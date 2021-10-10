use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

pub use crate::ffi::system::sfTime as Time;

impl Time {
    /// Constructs a time value from a number of seconds.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn seconds(seconds: f32) -> Self {
        Time{microseconds: (seconds * 1000000.) as i64}
    }

    /// Constructs a time value from a number of milliseconds.
    #[must_use]
    pub const fn milliseconds(milliseconds: i32) -> Self {
        Time{microseconds: milliseconds as i64 * 1000}
    }

    /// Constructs a time value from a number of microseconds.
    #[must_use]
    pub const fn microseconds(microseconds: i64) -> Self {
        Time{microseconds}
    }

    /// Returns the time value as a number of seconds.
    #[must_use]
    pub fn as_seconds(self) -> f32 {
        self.microseconds as f32 / 1000000.
    }

    /// Returns the time value as a number of milliseconds.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn as_milliseconds(self) -> i32 {
        (self.microseconds / 1000) as i32
    }

    /// Returns the time value as a number of microseconds.
    #[must_use]
    pub fn as_microseconds(self) -> i64 {
        self.microseconds
    }

    /// Predefined "zero" time value.
    pub const ZERO: Time = Time{microseconds: 0};
}

impl Neg for Time {
    type Output = Self;
    fn neg(self) -> Self {
        Time{microseconds: -self.microseconds}
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Time::microseconds(self.microseconds + rhs.microseconds)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.microseconds += rhs.microseconds;
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Time::microseconds(self.microseconds - rhs.microseconds)
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        self.microseconds -= rhs.microseconds;
    }
}

impl Mul<f32> for Time {
    type Output = Self;

    /// Overload of binary * operator to scale a time value.
    fn mul(self, rhs: f32) -> Self {
        Time::seconds(self.as_seconds() * rhs)
    }
}

impl Mul<i64> for Time {
    type Output = Self;

    /// Overload of binary * operator to scale a time value.
    fn mul(self, rhs: i64) -> Self {
        Time::microseconds(self.as_microseconds() * rhs)
    }
}

impl Mul<Time> for f32 {
    type Output = Time;

    /// Overload of binary * operator to scale a time value.
    fn mul(self, rhs: Time) -> Time {
        rhs * self
    }
}

impl Mul<Time> for i64 {
    type Output = Time;

    /// Overload of binary * operator to scale a time value.
    fn mul(self, rhs: Time) -> Time {
        rhs * self
    }
}

impl MulAssign<f32> for Time {
    /// Overload of binary *= operator to scale/assign a time value.
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl MulAssign<i64> for Time {
    /// Overload of binary *= operator to scale/assign a time value.
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Time {
    type Output = Self;

    /// Overload of binary / operator to scale a time value.
    fn div(self, rhs: f32) -> Self {
        Time::seconds(self.as_seconds() / rhs)
    }
}

impl Div<i64> for Time {
    type Output = Self;

    /// Overload of binary / operator to scale a time value.
    fn div(self, rhs: i64) -> Self {
        Time::microseconds(self.as_microseconds() / rhs)
    }
}

impl Div for Time {
    type Output = f32;

    /// Overload of binary / operator to compute the ratio of two time values.
    fn div(self, rhs: Self) -> f32 {
        self.as_seconds() / rhs.as_seconds()
    }
}

impl DivAssign<f32> for Time {
    /// Overload of binary /= operator to scale/assign a time value.
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl DivAssign<i64> for Time {
    /// Overload of binary /= operator to scale/assign a time value.
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

impl Rem for Time {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Time::microseconds(self.microseconds % rhs.microseconds)
    }
}

impl RemAssign for Time {
    fn rem_assign(&mut self, rhs: Self) {
        self.microseconds %= rhs.microseconds
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::ZERO
    }
}

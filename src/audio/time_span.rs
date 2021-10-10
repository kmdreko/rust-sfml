use crate::{ffi, system::Time};

#[derive(Default, Debug, Clone, Copy)]
/// Structure defining a time range
pub struct TimeSpan {
    /// The beginning offset of the time range.
    pub offset: Time,
    /// The length of the time range.
    pub length: Time,
}

impl TimeSpan {
    pub(crate) fn from_raw(raw: ffi::sfTimeSpan) -> Self {
        Self {
            offset: Time{microseconds: raw.offset},
            length: Time{microseconds: raw.length},
        }
    }
    pub(crate) fn into_raw(self) -> ffi::sfTimeSpan {
        ffi::sfTimeSpan {
            offset: self.offset.as_microseconds(),
            length: self.length.as_microseconds(),
        }
    }
}

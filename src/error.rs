use alloc::boxed::Box;
use core::fmt;

#[derive(Debug)]
pub enum ZError {
    InvalidLength(usize, usize),
    OutOfBounds(usize, usize),
    InvalidUtf8,
    Custom(&'static str),
    Context {
        label: &'static str,
        offset: usize,
        source: Box<ZError>,
    },
}

impl ZError {
    #[inline]
    pub fn with_context(self, label: &'static str, offset: usize) -> Self {
        Self::Context {
            label,
            offset,
            source: Box::new(self),
        }
    }
}

impl fmt::Display for ZError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZError::InvalidLength(expected, actual) => {
                write!(f, "Invalid length: expected {}, got {}", expected, actual)
            }
            ZError::OutOfBounds(idx, len) => {
                write!(f, "Index out of bounds: index {}, len {}", idx, len)
            }
            ZError::InvalidUtf8 => write!(f, "Invalid UTF-8 sequence"),
            ZError::Custom(msg) => write!(f, "Error: {}", msg),
            ZError::Context {
                label,
                offset,
                source,
            } => {
                write!(
                    f,
                    "Failed to decode {} at byte offset {}: {}",
                    label, offset, source
                )
            }
        }
    }
}

// In no_std, we don't have std::error::Error, so we just stick to Display + Debug.
// If we wanted to support standard Error trait when std is enabled, we could use cfg features.

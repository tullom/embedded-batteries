/// Smart Battery error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic Smart Battery error kind.
    ///
    /// By using this method, Smart Battery errors freely defined by HAL implementations
    /// can be converted to a set of generic Smart Battery errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// Smart Battery error kind.
///
/// This represents a common set of Smart Battery operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common Smart Battery errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// An error occurred on the underlying peripheral supporting the sensor.
    /// e.g. An I2C bus error occurs for an I2C enabled Smart Battery.
    /// The original error may contain more information.
    CommError,
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CommError => write!(f, "Error communicating with Smart Battery"),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Smart Battery error type trait.
///
/// This just defines the error type, to be used by the other Smart Battery traits.
pub trait ErrorType {
    /// Error type.
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

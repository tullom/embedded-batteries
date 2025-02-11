use crate::{MilliAmps, MilliVolts};

/// Charger error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic charger error kind.
    ///
    /// By using this method, charger errors freely defined by HAL implementations
    /// can be converted to a set of generic charger errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// Charger error kind.
///
/// This represents a common set of charger operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common charger errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// An error occurred on the underlying peripheral supporting the sensor.
    /// e.g. An I2C bus error occurs for an I2C enabled smart charger.
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
            Self::CommError => write!(f, "Error communicating with charger"),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Charger error type trait.
///
/// This just defines the error type, to be used by the other Charger traits.
pub trait ErrorType {
    /// Error type.
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

/// Blocking Smart Battery Charger methods
pub trait Charger: ErrorType {
    /// Sets the maximum current that a Smart Battery Charger may deliver to
    /// the Smart Battery. Returns charge current as acknowledged by the charger.
    /// In combination with the ChargingVoltage() function and the battery's internal
    /// impedance, this function determines the Smart Battery Charger's desired operating point. Together, these
    /// functions permit a Smart Battery Charger to dynamically adjust its charging profile (current/voltage) for
    /// optimal charge. The Smart Battery can effectively turn off the Smart Battery Charger by returning a value
    /// of 0 for this function.
    fn charging_current(&mut self, current: MilliAmps) -> Result<MilliAmps, Self::Error>;

    /// Sets and returns the maximum voltage that a Smart Battery Charger may deliver to the
    /// Smart Battery. Returns charge current as acknowledged by the charger.
    /// In combination with the ChargingCurrent() function and the battery's internal impedance,
    /// this function determines the Smart Battery Charger's desired operating point. Together, these functions
    /// permit a Smart Battery Charger to dynamically adjust its charging profile (current/voltage) for optimal
    /// charge. The Smart Battery can effectively turn off the Smart Battery Charger by returning a value of 0 for
    /// this function.
    fn charging_voltage(&mut self, voltage: MilliVolts) -> Result<MilliVolts, Self::Error>;
}

impl<T: Charger + ?Sized> Charger for &mut T {
    #[inline]
    fn charging_current(&mut self, current: MilliAmps) -> Result<MilliAmps, Self::Error> {
        T::charging_current(self, current)
    }

    #[inline]
    fn charging_voltage(&mut self, voltage: MilliVolts) -> Result<MilliVolts, Self::Error> {
        T::charging_voltage(self, voltage)
    }
}

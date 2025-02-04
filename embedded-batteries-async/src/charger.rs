use core::future::Future;

pub use embedded_batteries::charger::{Error, ErrorKind, ErrorType, MilliAmps, MilliVolts};

/// Asynchronous Smart Battery Charger methods
pub trait Charger: ErrorType {
    /// Asynchronously sets the maximum current that a Smart Battery Charger may deliver to
    /// the Smart Battery. Returns charge current as acknowledged by the charger.
    /// In combination with the ChargingVoltage() function and the battery's internal
    /// impedance, this function determines the Smart Battery Charger's desired operating point. Together, these
    /// functions permit a Smart Battery Charger to dynamically adjust its charging profile (current/voltage) for
    /// optimal charge. The Smart Battery can effectively turn off the Smart Battery Charger by returning a value
    /// of 0 for this function.
    fn charging_current(&mut self, current: MilliAmps) -> impl Future<Output = Result<MilliAmps, Self::Error>>;

    /// Asynchronously sets and returns the maximum voltage that a Smart Battery Charger may deliver to the
    /// Smart Battery. Returns charge current as acknowledged by the charger.
    /// In combination with the ChargingCurrent() function and the battery's internal impedance,
    /// this function determines the Smart Battery Charger's desired operating point. Together, these functions
    /// permit a Smart Battery Charger to dynamically adjust its charging profile (current/voltage) for optimal
    /// charge. The Smart Battery can effectively turn off the Smart Battery Charger by returning a value of 0 for
    /// this function.
    fn charging_voltage(&mut self, voltage: MilliVolts) -> impl Future<Output = Result<MilliVolts, Self::Error>>;
}

impl<T: Charger + ?Sized> Charger for &mut T {
    #[inline]
    async fn charging_current(&mut self, current: MilliAmps) -> Result<MilliAmps, Self::Error> {
        T::charging_current(self, current).await
    }

    #[inline]
    async fn charging_voltage(&mut self, voltage: MilliVolts) -> Result<MilliVolts, Self::Error> {
        T::charging_voltage(self, voltage).await
    }
}

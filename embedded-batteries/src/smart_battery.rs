use bitfield_struct::bitfield;

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
    /// An error occured and was reported by a read from the BatteryStatus (0x16) register.
    BatteryStatus(ErrorCode),
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
            Self::BatteryStatus(_) => write!(
                f,
                "Error reported by BatteryService (0x16) register. The original error may contain more information"
            ),
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

/// Depending on the value of the CapacityMode bit, the Smart Battery will use milliamps or centiwatts.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapacityModeValue {
    /// Unsigned Milliamp or MilliampHour representation, used when CapacityMode bit = 0.
    MilliAmpUnsigned(u16),
    /// Unsigned Centiwatt or CentiwattHour representation, used when CapacityMode bit = 1.
    CentiWattUnsigned(u16),
}

/// Time is measured in minutes, where 1 minute is 1
pub type Minutes = u16;

/// Depending on the value of the CapacityMode bit, the Smart Battery will use milliamps or centiwatts.
/// Signed to represent negative currents and capacities.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapacityModeSignedValue {
    /// Signed Milliamp or MilliampHour representation, used when CapacityMode bit = 0.
    MilliAmpSigned(i16),
    /// Signed Milliamp or MilliampHour representation, used when CapacityMode bit = 1.
    CentiWattSigned(i16),
}

/// Temperature is measured in decikelvins, where 0.1 Kelvin is 1.
pub type DeciKelvin = u16;

/// Percent, 1% is 1.
pub type Percent = u8;

/// Cycles, 1 cycle is 1.
pub type Cycles = u16;

/// Error codes that must be supported by the Smart Battery.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ErrorCode {
    /// The Smart Battery processed the function code
    /// without detecting any errors.
    Ok = 0,

    /// The Smart Battery is unable to process the function
    /// code at this time.
    Busy = 1,

    /// The Smart Battery detected an attempt to read or
    /// write to a function code reserved by this version of
    /// the specification. The Smart Battery detected an
    /// attempt to access an unsupported optional
    /// manufacturer function code.
    ReservedCmd = 2,

    /// The Smart Battery does not support this function
    /// code which is defined in this version of the
    /// specification.
    UnsupportedCmd = 3,

    /// The Smart Battery detected an attempt to write to a
    /// read only function code.
    AccessDenied = 4,

    /// The Smart Battery detected a data overflow or
    /// under flow.
    UnderOverFlow = 5,

    /// The Smart Battery detected an attempt to write to a
    /// function code with an incorrect size data block.
    BadSize = 6,

    /// The Smart Battery detected an unidentifiable error.
    UnknownError = 7,
}

impl ErrorCode {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Ok,
            1 => Self::Busy,
            2 => Self::ReservedCmd,
            3 => Self::UnsupportedCmd,
            4 => Self::AccessDenied,
            5 => Self::UnderOverFlow,
            6 => Self::BadSize,
            _ => Self::UnknownError,
        }
    }
}

/// Revision of SBS Spec, used in specification_info().
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Revision {
    /// Version 1.0 and 1.1.
    Version1And1Dot1 = 1,
}

impl Revision {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            1 => Self::Version1And1Dot1,
            _ => unreachable!(),
        }
    }
}

/// Version of SBS Spec, used in specification_info().
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Version {
    /// Reserved.
    Reserved = 0,

    /// Version 1.0.
    Version1 = 1,

    /// Version 1.1.
    Version1Dot1 = 2,

    /// Version 1.1 with optional PEC support.
    Version1Dot1Pec = 3,
}

impl Version {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            1 => Self::Version1,
            2 => Self::Version1Dot1,
            3 => Self::Version1Dot1Pec,
            _ => Self::Reserved,
        }
    }
}

/// Return value of the manufacture_date() function (0x1b). The date is packed in the
/// following fashion: (year-1980) * 512 + month * 32 + day.
#[bitfield(u16, defmt = cfg(feature = "defmt"))]
pub struct ManufactureDate {
    /// 1 - 31 (corresponds to date).
    #[bits(5)]
    pub day: usize,

    /// 1 - 12 (corresponds to month number).
    #[bits(4)]
    pub month: usize,

    /// 0 - 127 (corresponds to year biased by 1980).
    /// Add 1980 to the year to get the true year.
    #[bits(7)]
    pub year: usize,
}

/// Return value of the battery_mode() function (0x03). See the SBS spec for more information.
#[bitfield(u16, defmt = cfg(feature = "defmt"))]
pub struct BatteryModeFields {
    /// INTERNAL_CHARGE_CONTROLLER bit set indicates that the battery pack contains its own internal
    /// charge controller. When the bit is set, this optional function is supported and the
    /// CHARGE_CONTROLLER_ENABLED bit will be available for activation and control of the actual
    /// internal charger.
    #[bits(1, access = RO)]
    pub internal_charge_controller: bool,

    /// PRIMARY_BATTERY_SUPPORT bit set indicates that the battery pack has the ability to act as either
    /// the primary or secondary battery in a system. When the bit is set, this function is supported and the
    /// PRIMARY_BATTERY bit will be available for activation and control of this function
    #[bits(1, access = RO)]
    pub primary_battery_support: bool,

    #[bits(5)]
    __: u8,

    /// CONDITION_FLAG bit set indicates that the battery is requesting a conditioning cycle. A conditioning
    /// cycle may be requested because of the characteristics of the battery chemistry and/or the electronics in
    /// combination with the usage pattern.
    #[bits(1, access = RO)]
    pub condition_flag: bool,

    /// CHARGE_CONTROLLER_ENABLED bit is set to enable the battery pack’s internal charge controller.
    /// When this bit is cleared, the internal charge controller is disabled (default). This bit is active only when the
    /// INTERNAL_CHARGE_CONTROLLER bit is set, indicating that this function is supported. The status of
    /// a battery pack’s internal charge controller can be determined by reading this bit
    pub charge_controller_enabled: bool,

    /// PRIMARY_BATTERY bit is set to enable a battery to operate as the primary battery in a system. When
    /// this bit is cleared, the battery operates in a secondary role (default). This bit is active only when the
    /// PRIMARY_BATTERY_SUPPORT bit is set. The role that the battery is playing can be determined by
    /// reading this bit.
    pub primary_battery: bool,

    #[bits(3)]
    __: u8,

    /// ALARM_MODE bit is set to disable the Smart Battery's ability to master the SMBus and send
    /// AlarmWarning() messages to the SMBus Host and the Smart Battery Charger. When set, the Smart Battery
    /// will NOT master the SMBus and AlarmWarning() messages will NOT be sent to the SMBus Host and the
    /// Smart Battery Charger for a period of no more than 65 seconds and no less than 45 seconds. When
    /// cleared (default), the Smart Battery WILL send the AlarmWarning() messages to the SMBus Host and the
    /// Smart Battery Charger any time an alarm condition is detected. (See also Section 5.4 of the SBS spec
    /// for a more detailed explanation of alarm conditions and operations.)
    ///
    /// When the ALARM_MODE bit is set, the system assumes responsibility for detecting and
    /// responding to Smart Battery alarms by reading the BatteryStatus() to determine if any of the alarm
    /// bit flags are set. At a minimum, this requires the system to poll the Smart Battery BatteryStatus()
    /// every 10 seconds at all times the SMBus is active. The system is expected to take appropriate
    /// action.
    ///
    /// The ALARM_MODE bit is automatically cleared by the Smart Battery electronics every 60
    /// seconds so that any accidental activation of this mode will not be persistent. A SMBus Host which
    /// does not want the Smart Battery to be a master on the SMBus must therefore continually set this
    /// bit at least once per 45 seconds to keep the ALARM_MODE bit set
    pub alarm_mode: bool,

    /// CHARGER_MODE bit enables or disables the Smart Battery's transmission of ChargingCurrent() and
    /// ChargingVoltage() messages to the Smart Battery Charger. When set, the Smart Battery will NOT transmit
    /// ChargingCurrent() and ChargingVoltage() values to the Smart Battery Charger. When cleared, the Smart
    /// Battery will transmit the ChargingCurrent() and ChargingVoltage() values to the Smart Battery Charger
    /// when charging is desired. (See Section 5.3 of the SBS spec for a more detailed explanation.)
    ///
    /// When the CHARGER_MODE bit is set, the system assumes responsibility for safely charging the
    /// Smart Battery. At a minimum, this requires the system to poll the Smart Battery for
    /// ChargingVoltage() and ChargingCurrent() at the same rate the Smart Battery would normally send
    /// these charging messages to the Smart Battery Charger (e.g. every 5 seconds to 60 seconds.)
    /// The CHARGER_MODE bit allows a SMBus Host or Smart Battery Charger to disable the Smart
    /// Battery's broadcast of the ChargingCurrent() and ChargingVoltage().
    ///
    /// The use of CHARGER_MODE does NOT affect the use of ALARM_MODE. If only
    /// CHARGER_MODE bit is set, AlarmWarning messages relating to charging will still occur and be
    /// broadcast the Smart Battery Charger and SMBus Host. (See ALARM_MODE bit flag definition.)
    pub charger_mode: bool,

    /// CAPACITY_MODE bit indicates if capacity information will be reported in mA/mAh or 10mW/10mWh.
    /// When set, the capacity information will be reported in 10mW/10mWh as appropriate. When cleared, the
    /// capacity information will be reported in mA/mAh as appropriate.
    ///
    /// After changing the CAPACITY_MODE bit, all related values (such as AtRate()) should be re-written while
    /// the new mode is active. This is because changes made to the CAPACITY_MODE bit do not retroactively
    /// affect values which may have been previously written in another mode. For example, a value written to
    /// AtRate() while the CAPACITY_MODE bit was 0 will cause AtRate calculations to be made using the mAH
    /// value. Changing the CAPACITY_MODE bit to 1 will not automatically cause all the AtRate calculations to
    /// be re-calculated using the 10mWH equivalent, although this is permitted, it is not required.
    pub capacity_mode: bool,
}

/// Return value of the battery_status() function (0x16). See the SBS spec for more information.
#[bitfield(u16, defmt = cfg(feature = "defmt"))]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BatteryStatusFields {
    /// Error codes from the Smart Battery. See ErrorCode enum fields for detailed documentation on what each
    /// error code entails.
    #[bits(4)]
    pub error_code: ErrorCode,

    /// FULLY_DISCHARGED bit is set when the Smart Battery determines that it has supplied all the charge it
    /// can. Discharge should be stopped soon.
    ///
    /// This bit will be cleared when the RelativeStateOfCharge() is
    /// greater than or equal to 20%. This status bit may be set prior to the
    /// ‘TERMINATE_DISCHARGE_ALARM’ as an early or first level warning of end of battery charge.
    pub fully_discharged: bool,

    /// FULLY_CHARGED bit is set when the Smart Battery determines that has reached a full charge point.
    ///
    ///  This bit will be cleared when the battery may want to be charged again, which is chemistry and
    /// manufacturer specific.
    pub fully_charged: bool,

    /// DISCHARGING bit is set when the Smart Battery determines that it is not being charged.
    /// This bit will be cleared when the battery detects that it is being charged.
    pub discharging: bool,

    /// INITIALIZED bit is SET when the Smart Battery electronics are calibrated or configured for the first time,
    /// typically at the time of battery pack assembly or manufacture.
    ///
    /// It will be cleared when the battery detects that this calibration or configuration data has been lost or
    /// altered and a significant degradation in accuracy is possible.
    ///
    /// The INITIALIZED status bit is the second and more serious signal from the Smart Battery that it has
    /// perhaps lost the ability to determine the present state-of-charge. As a result other data values required by
    /// this specification may be inaccurate.
    ///
    /// (The first signal from the Smart Battery is typically the CONDITION_FLAG found in the BatteryMode()
    /// register.)
    pub initialized: bool,

    /// REMAINING_TIME_ALARM bit is set when the Smart Battery detects that the estimated remaining
    /// time at the present discharge rate represented by the value in AverageTimeToEmpty() is less than that set by
    /// the RemainingTimeAlarm() function.
    ///
    /// This bit will be cleared when either the value set by the RemainingTimeAlarm() function is lower than the
    /// AverageTimeToEmpty() or when the AverageTimeToEmpty() is increased by charging the Smart Battery
    /// or decreasing the discharge rate.
    ///
    /// (NOTE: This Alarm bit can be disabled by writing zero to the RemainingTimeAlarm() value.)
    pub remaining_time_alarm: bool,

    /// REMAINING_CAPACITY_ALARM bit is set when the Smart Battery detects that its
    /// RemainingCapacity() is less than that set by the RemainingCapacityAlarm() function.
    ///
    /// This bit will be cleared when either the value set by the RemainingCapacityAlarm() function is lower than the
    /// RemainingCapacity() or when the RemainingCapacity() is increased by charging the Smart Battery.
    ///
    /// (NOTE: This Alarm bit can be disabled by writing zero to the RemainingCapacityAlarm() value.)
    pub remaining_capacity_alarm: bool,

    __: bool,

    /// TERMINATE_DISCHARGE_ALARM bit is set when the Smart Battery determines that it has supplied
    /// all the charge it can at the present discharge rate. Discharge should be stopped as soon as possible.
    ///
    /// This bit will be cleared when the Smart Battery detects that the discharge has stopped or that the rate
    /// has lessened.
    /// (Note that since this is rate dependent, it may occur at a high discharge rate and disappear when the
    /// discharge rate has slowed such that the Smart Battery can continue to be discharged at the lower rate.)
    pub terminate_discharge_alarm: bool,

    /// OVER_TEMP_ALARM bit will be set when the Smart Battery detects that its internal temperature is
    /// greater than a preset allowable limit. When this bit is set, charging should be stopped as soon as possible.
    /// The Smart Battery may not yet be in a Fully Charged state.
    /// Charging is effectively ‘suspended,’ usually temporarily. Charging may resume when the Smart Battery
    /// detects that its internal temperature is below a preset limit to allow charging again. (This limit may be a
    /// different value than what caused the original alarm.)
    ///
    /// This bit is cleared when the internal temperature has dropped below an acceptable limit, which may or may
    /// not be the original alarm threshold (although charging may not always resume at this point.)
    pub over_temp_alarm: bool,

    __: bool,

    /// TERMINATE_CHARGE_ALARM bit is set when charging should be stopped but the Smart Battery may
    /// not yet be in a Fully Charged state. Charging is effectively ‘suspended,’ usually temporarily. Charging may
    /// resume when the Smart Battery detects that its charging parameters are back in allowable ranges and
    /// ChargingVoltage() and ChargingCurrent() values are both returned to non-zero values.
    ///
    /// This bit is cleared when the Smart Battery detects that it is no longer being charged.
    pub terminate_charge_alarm: bool,

    /// OVER_CHARGED_ALARM bit is set whenever the Smart Battery detects that it is being charged
    /// beyond a Fully Charged state. When this bit is set, charging should be completely stopped as soon as
    /// possible. Charging further can result in permanent damage to the battery.
    ///
    /// This bit will be cleared when the Smart Battery detects that it is no longer being charged. Charging should
    /// not automatically restart.
    pub over_charged_alarm: bool,
}

/// Return value of the specification_info() function (0x1a). See the SBS spec for more information.
#[bitfield(u16, defmt = cfg(feature = "defmt"))]
pub struct SpecificationInfoFields {
    /// Revision of the SBS spec supported by this Smart Battery.
    /// See Revision enum fields for detailed documentation.
    #[bits(4)]
    pub revision: Revision,

    /// Version of the SBS spec supported by this Smart Battery.
    /// See Version enum fields for detailed documentation.
    #[bits(4)]
    pub version: Version,

    /// 0 - 3 (multiplies voltages* by 10 ^ VScale).
    #[bits(4)]
    pub v_scale: u8,

    /// 0 - 3 (multiplies currents* and capacities by 10 ^ IPScale).
    #[bits(4)]
    pub ip_scale: u8,
}

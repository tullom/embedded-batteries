use core::future::Future;

pub use embedded_batteries::smart_battery::{
    BatteryModeFields, BatteryStatusFields, CapacityModeSignedValue, CapacityModeValue, Cycles, DeciKelvin, Error,
    ErrorCode, ErrorKind, ErrorType, ManufactureDate, Minutes, Percent, Revision, SpecificationInfoFields, Version,
};
pub use embedded_batteries::{MilliAmpsSigned, MilliVolts};

/// Asynchronous Smart Battery methods.
pub trait SmartBattery: ErrorType {
    /// 0x01
    ///
    /// Sets or gets the Low Capacity alarm threshold value. Whenever the RemainingCapacity() falls below the
    /// Low Capacity value, the Smart Battery sends AlarmWarning() messages to the SMBus Host with the
    /// REMAINING_CAPACITY_ALARM bit set. A Low Capacity value of 0 disables this alarm.
    /// (If the ALARM_MODE bit is set in BatteryMode() then the AlarmWarning() message is disabled for a set
    /// period of time. See the BatteryMode() function for further information.)
    ///
    /// The Low Capacity value is set to 10% of design capacity at time of manufacture. The Low Capacity value
    /// will remain unchanged until altered by the RemainingCapacityAlarm() function. The Low Capacity value
    /// may be expressed in either capacity (mAh) or power (10mWh) depending on the setting of the
    /// BatteryMode()'s CAPACITY_MODE bit (see BatteryMode()).
    fn remaining_capacity_alarm(
        &mut self,
        capacity: CapacityModeValue,
    ) -> impl Future<Output = Result<CapacityModeValue, Self::Error>>;

    /// 0x02
    ///
    /// Sets or gets the Remaining Time alarm value. Whenever the AverageTimeToEmpty() falls below the
    /// Remaining Time value, the Smart Battery sends AlarmWarning() messages to the SMBus Host with the
    /// REMAINING_TIME_ALARM bit set. A Remaining Time value of 0 effectively disables this alarm.
    /// (If the ALARM_MODE bit is set in BatteryMode() then the AlarmWarning() message is disabled for a set
    /// period of time. See the BatteryMode() function for further information.)
    ///
    /// The Remaining Time value is set to 10 minutes at time of manufacture. The Remaining Time value will
    /// remain unchanged until altered by the RemainingTimeAlarm() function.
    fn remaining_time_alarm(&mut self, time: Minutes) -> impl Future<Output = Result<Minutes, Self::Error>>;

    /// 0x03
    ///
    /// This function selects the various battery operational modes and reports the battery’s capabilities, modes,
    /// and flags minor conditions requiring attention.
    ///
    /// See the SBS specification for detailed documentation.
    fn battery_mode(&mut self, flags: u16) -> impl Future<Output = Result<u16, Self::Error>>;

    /// 0x04
    ///
    /// The AtRate() function is the first half of a two-function call-set used to set the AtRate value used in
    /// calculations made by the AtRateTimeToFull(), AtRateTimeToEmpty(), and AtRateOK() functions. The
    /// AtRate value may be expressed in either current (mA) or power (10mW) depending on the setting of the
    /// BatteryMode()'s CAPACITY_MODE bit. (Configuration of the CAPACITY_MODE bit will alter the
    /// calculation of AtRate functions. Changing the state of CAPACITY_MODE may require a re-write to the
    /// AtRate() function using the appropriate units.)
    fn at_rate(
        &mut self,
        rate: CapacityModeSignedValue,
    ) -> impl Future<Output = Result<CapacityModeSignedValue, Self::Error>>;

    /// 0x05
    ///
    /// Returns the predicted remaining time to fully charge the battery at the previously written AtRate value in mA.
    ///
    /// Note: This function is only required to return a value when the CAPACITY_MODE bit is cleared and the
    /// AtRate() value is written in mA units. If the CAPACITY_MODE bit is set, then AtRateTimeToFull() may
    /// return 65535 to indicate over-range and return an error code indicating overflow. Alternately, this function
    /// may return a remaining time to full based on a 10 mW value in AtRate().
    fn at_rate_time_to_full(&mut self) -> impl Future<Output = Result<Minutes, Self::Error>>;

    /// 0x06
    ///
    /// Returns the predicted remaining operating time if the battery is discharged at the previously written AtRate
    /// value. (Result will depend on the setting of CAPACITY_MODE bit.)
    fn at_rate_time_to_empty(&mut self) -> impl Future<Output = Result<Minutes, Self::Error>>;

    /// 0x07
    ///
    /// Returns a Boolean value that indicates whether or not the battery can deliver the previously written AtRate
    /// value of additional energy for 10 seconds (Boolean). If the AtRate value is zero or positive, the
    /// AtRateOK() function will ALWAYS return true. Result may depend on the setting of CAPACITY_MODE
    /// bit.
    fn at_rate_ok(&mut self) -> impl Future<Output = Result<bool, Self::Error>>;

    /// 0x08
    ///
    /// Returns the cell-pack's internal temperature (°K). The actual operational temperature range will be defined
    /// at a pack level by a particular manufacturer.
    fn temperature(&mut self) -> impl Future<Output = Result<bool, Self::Error>>;

    /// 0x09
    ///
    /// Returns the cell-pack voltage (mV).
    fn voltage(&mut self) -> impl Future<Output = Result<MilliVolts, Self::Error>>;

    /// 0x0A
    ///
    /// Returns the current being supplied (or accepted) through the battery's terminals (mA).
    fn current(&mut self) -> impl Future<Output = Result<MilliAmpsSigned, Self::Error>>;

    /// 0x0B
    ///
    /// Returns a one-minute rolling average based on the current being supplied (or accepted) through the battery's
    /// terminals (mA). The AverageCurrent() function is expected to return meaningful values during the battery's
    /// first minute of operation.
    fn average_current(&mut self) -> impl Future<Output = Result<MilliAmpsSigned, Self::Error>>;

    /// 0x0C
    ///
    /// Returns the expected margin of error (%) in the state of charge calculation. For example, when MaxError()
    /// returns 10% and RelativeStateOfCharge() returns 50%, the Relative StateOfCharge() is actually between 50
    /// and 60%. The MaxError() of a battery is expected to increase until the Smart Battery identifies a condition
    /// that will give it higher confidence in its own accuracy. For example, when a Smart Battery senses that it has
    /// been fully charged from a fully discharged state, it may use that information to reset or partially reset
    /// MaxError(). The Smart Battery can signal when MaxError() has become too high by setting the
    /// CONDITION_FLAG bit in BatteryMode().
    fn max_error(&mut self) -> impl Future<Output = Result<Percent, Self::Error>>;

    /// 0x0D
    ///
    /// Returns the predicted remaining battery capacity expressed as a percentage of FullChargeCapacity() (%).
    fn relative_state_of_charge(&mut self) -> impl Future<Output = Result<Percent, Self::Error>>;

    /// 0x0E
    ///
    /// Returns the predicted remaining battery capacity expressed as a percentage of DesignCapacity() (%).
    ///
    /// Note that AbsoluteStateOfCharge() can return values greater than 100%.
    fn absolute_state_of_charge(&mut self) -> impl Future<Output = Result<Percent, Self::Error>>;

    /// 0x0F
    ///
    /// Returns the predicted remaining battery capacity. The RemainingCapacity() capacity value is expressed in
    /// either current (mAh at a C/5 discharge rate) or power (10mWh at a P/5 discharge rate) depending on the
    /// setting of the BatteryMode()'s CAPACITY_MODE bit.
    fn remaining_capacity(&mut self) -> impl Future<Output = Result<CapacityModeValue, Self::Error>>;

    /// 0x10
    ///
    /// Returns the predicted pack capacity when it is fully charged. The FullChargeCapacity() value is expressed
    /// in either current (mAh at a C/5 discharge rate) or power (10mWh at a P/5 discharge rate) depending on the
    /// setting of the BatteryMode()'s CAPACITY_MODE bit.
    fn full_charge_capacity(&mut self) -> impl Future<Output = Result<CapacityModeValue, Self::Error>>;

    /// 0x11
    ///
    /// Returns the predicted remaining battery life at the present rate of discharge (minutes). The
    /// RunTimeToEmpty() value is calculated based on either current or power depending on the setting of the
    /// BatteryMode()'s CAPACITY_MODE bit. This is an important distinction because use of the wrong
    /// calculation mode may result in inaccurate return values.
    ///
    /// 65,535 indicates battery is not being discharged.
    fn run_time_to_empty(&mut self) -> impl Future<Output = Result<Minutes, Self::Error>>;

    /// 0x12
    ///
    /// Returns a one-minute rolling average of the predicted remaining battery life (minutes). The
    /// AverageTimeToEmpty() value is calculated based on either current or power depending on the setting of
    /// the BatteryMode()'s CAPACITY_MODE bit. This is an important distinction because use of the wrong
    /// calculation mode may result in inaccurate return values.
    ///
    /// 65,535 indicates battery is not being discharged.
    fn average_time_to_empty(&mut self) -> impl Future<Output = Result<Minutes, Self::Error>>;

    /// 0x13
    ///
    /// Returns a one minute rolling average of the predicted remaining time until the Smart Battery reaches full
    /// charge (minutes).
    ///
    /// 65,535 indicates the battery is not being charged.
    fn average_time_to_full(&mut self) -> impl Future<Output = Result<Minutes, Self::Error>>;

    /// 0x16
    ///
    /// Returns the Smart Battery's status word which contains Alarm and Status bit flags. Some of the
    /// BatteryStatus() flags (REMAINING_CAPACITY_ALARM and REMAINING_TIME_ALARM) are
    /// calculated based on either current or power depending on the setting of the BatteryMode()'s
    /// CAPACITY_MODE bit. This is important because use of the wrong calculation mode may result in an
    /// inaccurate alarm.
    fn battery_status(&mut self) -> impl Future<Output = Result<BatteryStatusFields, Self::Error>>;

    /// 0x17
    ///
    /// Returns the number of cycles the battery has experienced. A cycle is defined as:
    ///
    /// An amount of discharge approximately equal to the value of DesignCapacity.
    fn cycle_count(&mut self) -> impl Future<Output = Result<Cycles, Self::Error>>;

    /// 0x18
    ///
    /// Returns the theoretical capacity of a new pack. The DesignCapacity() value is expressed in either current
    /// (mAh at a C/5 discharge rate) or power (10mWh at a P/5 discharge rate) depending on the setting of the
    /// BatteryMode()'s CAPACITY_MODE bit.
    fn design_capacity(&mut self) -> impl Future<Output = Result<CapacityModeValue, Self::Error>>;

    /// 0x19
    ///
    /// Returns the theoretical voltage of a new pack (mV).
    fn design_voltage(&mut self) -> impl Future<Output = Result<MilliVolts, Self::Error>>;

    /// 0x1A
    ///
    /// Returns the version number of the Smart Battery specification the battery pack supports, as well as voltage
    /// and current and capacity scaling information in a packed unsigned integer. Power scaling is the product of
    /// the voltage scaling times the current scaling.
    /// These scaling functions do NOT affect ChargingCurrent() and ChargingVoltage() values.
    /// A Smart Battery Charger cannot be assumed to know this scaling information. (However, a ‘Level 3’
    /// or ‘Host Controlled’ Smart Battery Charger may read this value if required for specific
    /// applications.)
    fn specification_info(&mut self) -> impl Future<Output = Result<u16, Self::Error>>;

    /// 0x1B
    ///
    /// This function returns the date the cell pack was manufactured.
    fn manufacture_date(&mut self) -> impl Future<Output = Result<ManufactureDate, Self::Error>>;

    /// 0x1C
    ///
    /// This function is used to return a serial number. This number when combined with the ManufacturerName(),
    /// the DeviceName(), and the ManufactureDate() will uniquely identify the battery (unsigned int).
    fn serial_number(&mut self) -> impl Future<Output = Result<u16, Self::Error>>;

    /// 0x20
    ///
    /// This function accepts a mutable buffer of u8s and returns it filled with a **null-terminated** character array
    /// containing the battery's manufacturer's name. For example, "MyBattCo\0" would identify the Smart Battery's
    /// manufacturer as MyBattCo.
    fn manufacturer_name(&mut self, name: &mut [u8]) -> impl Future<Output = Result<(), Self::Error>>;

    /// 0x21
    ///
    /// This function accepts a mutable buffer of u8s and returns it filled with a **null-terminated** character array
    /// that contains the battery's name. For example, a DeviceName() of "MBC101\0" would indicate that
    /// the battery is a model MBC101.
    fn device_name(&mut self, name: &mut [u8]) -> impl Future<Output = Result<(), Self::Error>>;

    /// 0x22
    ///
    /// This function accepts a mutable buffer of u8s and returns it filled with a **null-terminated** character array
    /// that contains the battery's chemistry. For example, if the DeviceChemistry() function returns "NiMH\0",
    /// the battery pack would contain nickel metal hydride cells.
    fn device_chemistry(&mut self, chemistry: &mut [u8]) -> impl Future<Output = Result<(), Self::Error>>;
}

impl<T: SmartBattery + ?Sized> SmartBattery for &mut T {
    #[inline]
    async fn remaining_capacity_alarm(
        &mut self,
        capacity: CapacityModeValue,
    ) -> Result<CapacityModeValue, Self::Error> {
        T::remaining_capacity_alarm(self, capacity).await
    }

    #[inline]
    async fn remaining_time_alarm(&mut self, time: Minutes) -> Result<Minutes, Self::Error> {
        T::remaining_time_alarm(self, time).await
    }

    #[inline]
    async fn battery_mode(&mut self, flags: u16) -> Result<u16, Self::Error> {
        T::battery_mode(self, flags).await
    }

    #[inline]
    async fn at_rate(&mut self, rate: CapacityModeSignedValue) -> Result<CapacityModeSignedValue, Self::Error> {
        T::at_rate(self, rate).await
    }

    #[inline]
    async fn at_rate_time_to_full(&mut self) -> Result<Minutes, Self::Error> {
        T::at_rate_time_to_full(self).await
    }

    #[inline]
    async fn at_rate_time_to_empty(&mut self) -> Result<Minutes, Self::Error> {
        T::at_rate_time_to_empty(self).await
    }

    #[inline]
    async fn at_rate_ok(&mut self) -> Result<bool, Self::Error> {
        T::at_rate_ok(self).await
    }

    #[inline]
    async fn temperature(&mut self) -> Result<bool, Self::Error> {
        T::temperature(self).await
    }

    #[inline]
    async fn voltage(&mut self) -> Result<MilliVolts, Self::Error> {
        T::voltage(self).await
    }

    #[inline]
    async fn current(&mut self) -> Result<MilliAmpsSigned, Self::Error> {
        T::current(self).await
    }

    #[inline]
    async fn average_current(&mut self) -> Result<MilliAmpsSigned, Self::Error> {
        T::average_current(self).await
    }

    #[inline]
    async fn max_error(&mut self) -> Result<Percent, Self::Error> {
        T::max_error(self).await
    }

    #[inline]
    async fn relative_state_of_charge(&mut self) -> Result<Percent, Self::Error> {
        T::relative_state_of_charge(self).await
    }

    #[inline]
    async fn absolute_state_of_charge(&mut self) -> Result<Percent, Self::Error> {
        T::absolute_state_of_charge(self).await
    }

    #[inline]
    async fn remaining_capacity(&mut self) -> Result<CapacityModeValue, Self::Error> {
        T::remaining_capacity(self).await
    }

    #[inline]
    async fn full_charge_capacity(&mut self) -> Result<CapacityModeValue, Self::Error> {
        T::full_charge_capacity(self).await
    }

    #[inline]
    async fn run_time_to_empty(&mut self) -> Result<Minutes, Self::Error> {
        T::run_time_to_empty(self).await
    }

    #[inline]
    async fn average_time_to_empty(&mut self) -> Result<Minutes, Self::Error> {
        T::average_time_to_empty(self).await
    }

    #[inline]
    async fn average_time_to_full(&mut self) -> Result<Minutes, Self::Error> {
        T::average_time_to_full(self).await
    }

    #[inline]
    async fn battery_status(&mut self) -> Result<BatteryStatusFields, Self::Error> {
        T::battery_status(self).await
    }
    #[inline]
    async fn cycle_count(&mut self) -> Result<Cycles, Self::Error> {
        T::cycle_count(self).await
    }

    #[inline]
    async fn design_capacity(&mut self) -> Result<CapacityModeValue, Self::Error> {
        T::design_capacity(self).await
    }

    #[inline]
    async fn design_voltage(&mut self) -> Result<MilliVolts, Self::Error> {
        T::design_voltage(self).await
    }

    #[inline]
    async fn specification_info(&mut self) -> Result<u16, Self::Error> {
        T::specification_info(self).await
    }

    #[inline]
    async fn manufacture_date(&mut self) -> Result<ManufactureDate, Self::Error> {
        T::manufacture_date(self).await
    }

    #[inline]
    async fn serial_number(&mut self) -> Result<u16, Self::Error> {
        T::serial_number(self).await
    }

    #[inline]
    async fn manufacturer_name(&mut self, name: &mut [u8]) -> Result<(), Self::Error> {
        T::manufacturer_name(self, name).await
    }

    #[inline]
    async fn device_name(&mut self, name: &mut [u8]) -> Result<(), Self::Error> {
        T::device_name(self, name).await
    }

    #[inline]
    async fn device_chemistry(&mut self, chemistry: &mut [u8]) -> Result<(), Self::Error> {
        T::device_chemistry(self, chemistry).await
    }
}

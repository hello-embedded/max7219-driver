use crate::global::enums::{ DriverError, RegisterAddr};

/// Describes the interface used to connect to the MX7219
pub trait Connector {
    ///
    /// Get the total number of connected devices.
    /// # e.g.
    /// * 8*8 max7219 amount
    ///
    fn devices(&self) -> usize;

    ///
    /// Get the writes data max bytes.
    ///
    /// Serial-Data Format (16 Bits) [u8; 2]
    ///
    #[inline]
    fn serial_data_max_bytes(&self) -> usize {
        self.devices() * 2
    }

    ///
    /// Writes data to register.[`crate::global::enums::RegisterAddr`]
    ///
    /// Detail doc in [`crate::connectors::traits::Connector::write_raw`]
    ///
    fn write_register_data(
        &mut self,
        device_addr: usize,
        register_address: RegisterAddr,
        data: u8,
    ) -> Result<(), DriverError> {
        self.write_raw(device_addr, register_address as u8, data)
    }

    ///
    /// Writes data to given register address
    ///
    /// # Arguments
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    /// * `register_addr` - the command/register on the display to write to as u8
    /// * `data` - the data byte value to write
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned in case there was an error during data transfer
    ///
    fn write_raw(
        &mut self,
        device_addr: usize,
        register_addr: u8,
        data: u8,
    ) -> Result<(), DriverError>;
}

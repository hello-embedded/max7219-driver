use embedded_hal::blocking::spi::Write;

use crate::global::{consts::MAX_SERIAL_DATA_BYTES, enums::DriverError};

use super::traits::Connector;

pub struct SpiConnector<SPI>
where
    SPI: Write<u8>,
{
    devices: usize,
    buffer: [u8; MAX_SERIAL_DATA_BYTES],
    spi: SPI,
}

/// Hardware controlled CS connector with SPI transfer
impl<SPI> SpiConnector<SPI>
where
    SPI: Write<u8>,
{
    pub(crate) fn new(displays: usize, spi: SPI) -> Self {
        SpiConnector {
            devices: displays,
            buffer: [0; MAX_SERIAL_DATA_BYTES],
            spi,
        }
    }
}

impl<SPI> Connector for SpiConnector<SPI>
where
    SPI: Write<u8>,
{
    #[inline]
    fn devices(&self) -> usize {
        self.devices
    }
    fn write_raw(
        &mut self,
        device_addr: usize,
        register_addr: u8,
        data: u8,
    ) -> Result<(), DriverError> {
        // Each device has two elements([register_addr,data]) need * 2
        let offset = device_addr * 2;
        self.buffer = [0; MAX_SERIAL_DATA_BYTES];
        // set register_addr and data
        self.buffer[offset] = register_addr;
        self.buffer[offset + 1] = data;
        self.spi
            .write(&self.buffer[0..self.serial_data_max_bytes()])
            .map_err(|_| DriverError::Spi)?;

        Ok(())
    }
}

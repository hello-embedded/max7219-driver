use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v2::OutputPin;

use crate::global::enums::DriverError;

use super::spi::SpiConnector;
use super::traits::Connector;

/// Software controlled CS connector with SPI transfer
pub struct SpiConnectorCs<SPI, CS>
where
    SPI: Write<u8>,
    CS: OutputPin,
{
    spi_c: SpiConnector<SPI>,
    cs: CS,
}

impl<SPI, CS> SpiConnectorCs<SPI, CS>
where
    SPI: Write<u8>,
    CS: OutputPin,
{
    pub(crate) fn new(displays: usize, spi: SPI, cs: CS) -> Self {
        SpiConnectorCs {
            spi_c: SpiConnector::new(displays, spi),
            cs,
        }
    }
}

impl<SPI, CS> Connector for SpiConnectorCs<SPI, CS>
where
    SPI: Write<u8>,
    CS: OutputPin,
{
    fn devices(&self) -> usize {
        self.spi_c.devices()
    }

    fn write_raw(
        &mut self,
        device_addr: usize,
        register_addr: u8,
        data: u8,
    ) -> Result<(), DriverError> {
        self.cs.set_low().map_err(|_| DriverError::Pin)?;
        self.spi_c
            .write_raw(device_addr, register_addr, data)
            .map_err(|_| DriverError::Spi)?;
        self.cs.set_high().map_err(|_| DriverError::Pin)?;
        Ok(())
    }
}

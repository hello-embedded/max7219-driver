///
/// Possible Errors that can be raised either
/// during communication with the MAX7219 chip over SPI
/// or controlling the chip select pin.
///
#[derive(Debug)]
pub enum DriverError {
    /// An error occurred when working with SPI
    SpiError,
    /// An error occurred when working with a PIN
    PinError,
}

/// commands in the register map of the MAX7219.
pub enum Command {
    NoOp = 0x00,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Shutdown = 0x0C,
    DisplayTest = 0x0F,
}
/// the MAX7219 digit/row addresses
#[repr(u8)]
#[derive(Debug)]
pub enum DigitRowAddress {
    Digit0 = 0x01,
    Digit1 = 0x02,
    Digit2 = 0x03,
    Digit3 = 0x04,
    Digit4 = 0x05,
    Digit5 = 0x06,
    Digit6 = 0x07,
    Digit7 = 0x08,
}

// Implement TryFrom Trait on RowAddress to retrieve corresponding digit
impl TryFrom<u8> for DigitRowAddress {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use DigitRowAddress::*;

        Ok(match value {
            0x01 => Digit0,
            0x02 => Digit1,
            0x03 => Digit2,
            0x04 => Digit3,
            0x05 => Digit4,
            0x06 => Digit5,
            0x07 => Digit6,
            0x08 => Digit7,
            invalid => return Err(invalid),
        })
    }
}

/// the MAX7219 power modes.
pub enum Power {
    ShutdownMode = 0x00,
    NormalOperation = 0x01,
}

/// the MAX7219 decode modes for BCD encoded input.
pub enum DecodeMode {
    Nodecode = 0x00,
    CodeB0 = 0x01,
    CodeB30 = 0x0F,
    CodeB70 = 0xFF,
}

/// the MAX7219 supported LED intensity values.
pub enum Intensity {
    Min = 0x00,
    Ratio3_32 = 0x01,
    Ratio5_32 = 0x02,
    Ratio7_32 = 0x03,
    Ratio9_32 = 0x04,
    Ratio11_32 = 0x05,
    Ratio13_32 = 0x06,
    Ratio15_32 = 0x07,
    Ratio17_32 = 0x08,
    Ratio19_32 = 0x09,
    Ratio21_32 = 0x0A,
    Ratio23_32 = 0x0B,
    Ratio25_32 = 0x0C,
    Ratio27_32 = 0x0D,
    Ratio29_32 = 0x0E,
    Max = 0x0F,
}

/// the MAX7219 display scan limits
pub enum ScanLimit {
    Display0Only = 0x00,
    Display0And1 = 0x01,
    Display0To2 = 0x02,
    Display0To3 = 0x03,
    Display0To4 = 0x04,
    Display0To5 = 0x05,
    Display0To6 = 0x06,
    Display0To7 = 0x07,
}

/// the MAX7219 display test modes
pub enum DisplayTest {
    NormalOperationMode = 0x00,
    DisplayTestMode = 0x01,
}

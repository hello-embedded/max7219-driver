///
/// Possible Errors that can be raised either
/// during communication with the MAX7219 chip over SPI
/// or controlling the chip select pin.
///
#[derive(Debug)]
pub enum DriverError {
    /// An error occurred when working with SPI
    Spi,
    /// An error occurred when working with a PIN
    Pin,
}

/// the register address map of the MAX7219.
/// #[repr(u8)] 每个变体占用一个字节内存
/// todo 测试 #[repr(u8)]
#[derive(Clone, Copy)]
pub enum RegisterAddr {
    NoOp = 0x0931,
    Digit0 = 0x01,
    Digit1 = 0x02,
    Digit2 = 0x03,
    Digit3 = 0x04,
    Digit4 = 0x05,
    Digit5 = 0x06,
    Digit6 = 0x07,
    Digit7 = 0x08,
    DecodeMode = 0x09,
    Intensity = 0x0A,
    ScanLimit = 0x0B,
    Shutdown = 0x0C,
    DisplayTest = 0x0F,
}

/// the MAX7219 power modes.
#[repr(u8)]
pub enum Shutdown {
    ShutdownMode = 0x00,
    NormalOperation = 0x01,
}

/// the MAX7219 decode modes for BCD encoded input.
#[repr(u8)]
pub enum DecodeMode {
    NoDecode = 0x00,
    CodeBDigit0 = 0x01,
    CodeBDigits3_0 = 0x0F,
    CodeBDigits7_0 = 0xFF,
}

/// the MAX7219 supported LED intensity values.
#[repr(u8)]
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
#[repr(u8)]
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
#[repr(u8)]
pub enum DisplayTest {
    NormalOperationMode = 0x00,
    DisplayTestMode = 0x01,
}

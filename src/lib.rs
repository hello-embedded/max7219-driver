#![no_std]

pub mod connectors;
pub mod global;
use connectors::{spi::SpiConnector, spi_cs::SpiConnectorCs, traits::Connector};
use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v2::OutputPin;
use global::{consts::MAX_DIGITS, enums::*};

pub struct MAX7219<CONNECTOR> {
    c: CONNECTOR,
}

impl<CONNECTOR> MAX7219<CONNECTOR>
where
    CONNECTOR: Connector,
{
    // internal constructor, users should call ::from_pins or ::from_spi
    fn new(connector: CONNECTOR) -> Result<Self, DriverError> {
        let mut max7219 = MAX7219 { c: connector };
        max7219.init()?;
        Ok(max7219)
    }
    fn init(&mut self) -> Result<(), DriverError> {
        for i in 0..self.c.devices() {
            self.set_display_test_mode(i, DisplayTest::NormalOperationMode)?; // turn testmode off
            self.set_decode_mode(i, DecodeMode::NoDecode)?; // direct decode
            self.set_scan_limit(i, ScanLimit::Display0To7)?; // set scanlimit
            self.clear_display(i)?; // clear all digits
        }
        self.power_off()?; // power off
        Ok(())
    }

    ///
    /// Powers on all connected displays
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned in case there was an error during data transfer
    ///
    pub fn power_on(&mut self) -> Result<(), DriverError> {
        self.set_all_devices(RegisterAddr::Shutdown, Shutdown::NormalOperation as u8)
    }

    ///
    /// Powers off all connected displays
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned in case there was an error during data transfer
    ///
    pub fn power_off(&mut self) -> Result<(), DriverError> {
        self.set_all_devices(RegisterAddr::Shutdown, Shutdown::ShutdownMode as u8)
    }

    ///
    /// Configures the shutdown mode of the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `Shutdown` enum
    ///
    pub fn set_shutdown_mode(&mut self, mode: Shutdown) -> Result<(), DriverError> {
        self.set_all_devices(RegisterAddr::Shutdown, mode as u8)
    }

    ///
    /// Configures the decode mode on the input sent to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    /// * `mode` - one of the options in the `DecodeMode` enum
    ///
    pub fn set_decode_mode(
        &mut self,
        device_addr: usize,
        mode: DecodeMode,
    ) -> Result<(), DriverError> {
        self.c
            .write_register_data(device_addr, RegisterAddr::DecodeMode, mode as u8)
    }

    ///
    /// Configures the intensity of the LEDs on the display connected to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `Intensity` enum
    ///
    pub fn set_intensity_all(&mut self, mode: Intensity) -> Result<(), DriverError> {
        self.set_all_devices(RegisterAddr::Intensity, mode as u8)
    }
    ///
    /// Configures the intensity of the LEDs on the `device_addr` arg.
    ///
    /// # Arguments
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    /// * `mode` - one of the options in the `Intensity` enum
    ///
    pub fn set_intensity(
        &mut self,
        device_addr: usize,
        mode: Intensity,
    ) -> Result<(), DriverError> {
        self.c
            .write_register_data(device_addr, RegisterAddr::Intensity, mode as u8)
    }

    ///
    /// Configures the scanlimit for the MAX7219 IC.
    /// Applicable mostly to seven segment displays if certain digits (ex. on the left)
    /// need not to be shown.
    ///
    /// # Arguments
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    /// * `mode` - one of the options in the `ScanLimit` enum
    ///
    pub fn set_scan_limit(
        &mut self,
        device_addr: usize,
        mode: ScanLimit,
    ) -> Result<(), DriverError> {
        self.c
            .write_register_data(device_addr, RegisterAddr::ScanLimit, mode as u8)
    }

    ///
    /// Method to perform a visual test of the display.
    /// If performing a test, display needs to be put back in normal operation mode after done.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `DisplayTest` enum
    ///
    pub fn set_display_test_mode_all(&mut self, mode: DisplayTest) -> Result<(), DriverError> {
        self.set_all_devices(RegisterAddr::DisplayTest, mode as u8)
    }
    ///
    /// Method to perform a visual test of the display.
    /// If performing a test, display needs to be put back in normal operation mode after done.
    ///
    /// # Arguments
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    /// * `mode` - one of the options in the `Intensity` enum
    ///
    pub fn set_display_test_mode(
        &mut self,
        device_addr: usize,
        mode: DisplayTest,
    ) -> Result<(), DriverError> {
        self.c
            .write_register_data(device_addr, RegisterAddr::DisplayTest, mode as u8)
    }
    ///
    /// Method to clear the display.
    ///
    /// # Args
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    ///
    pub fn clear_display(&mut self, device_addr: usize) -> Result<(), DriverError> {
        for i in 1..9 {
            self.c.write_raw(device_addr, i, 0x00)?;
        }
        Ok(())
    }
    ///
    /// Method to clear the all display.
    ///
    /// # Args
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    ///
    pub fn clear_display_all(&mut self) -> Result<(), DriverError> {
        for i in 0..self.c.devices() {
            for j in 1..9 {
                self.c.write_raw(i, j, 0x00)?;
            }
        }
        Ok(())
    }

    ///
    /// Writes a raw value to the display
    ///
    /// # Arguments
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    /// * `register_addr` - display to address as connected in series (0 -> last)
    /// * `raw` - an array of raw bytes to write. Each bit represents a pixel on the display
    ///
    pub fn write_raw(
        &mut self,
        device_addr: usize,
        register_addr: RegisterAddr,
        data: u8,
    ) -> Result<(), DriverError> {
        self.c.write_register_data(device_addr, register_addr, data)
    }
    ///
    /// Writes a raw value to the display
    ///
    /// # Arguments
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    /// * `raw` - an array of raw bytes to write. Each bit represents a pixel on the display
    ///
    pub fn write_raw_all(
        &mut self,
        device_addr: usize,
        raw: &[u8; MAX_DIGITS],
    ) -> Result<(), DriverError> {
        let mut digit: u8 = 1;
        for b in raw {
            self.c.write_raw(device_addr, digit, *b)?;
            digit += 1;
        }
        Ok(())
    }
    ///
    /// Writes byte string to the display
    ///
    /// # Arguments
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    /// * `string` - the byte string to send 8 bytes long. Unknown characters result in question mark.
    /// * `dots` - u8 bit array specifying where to put dots in the string (1 = dot, 0 = not)
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned in case there was an error during data transfer
    ///
    /// # Examples
    ///
    /// ```
    /// let mut max7219 = MAX7219::from_spi_cs(1, spi, cs).unwrap();
    /// max7219.power_on();
    /// max7219.write_str(0, b"12345678", 0b0001_0000).unwrap();
    /// ```
    ///
    pub fn write_str(
        &mut self,
        device_addr: usize,
        string: &[u8; MAX_DIGITS],
        dots: u8,
    ) -> Result<(), DriverError> {
        self.set_decode_mode(0, DecodeMode::NoDecode)?;

        let mut digit: u8 = MAX_DIGITS as u8;
        let mut dot_product: u8 = 0b1000_0000;
        for b in string {
            let dot = (dots & dot_product) > 0;
            dot_product >>= 1;
            self.c.write_raw(device_addr, digit, ssb_byte(*b, dot))?;

            digit -= 1;
        }

        Ok(())
    }
    ///
    /// Writes BCD encoded string to the display
    ///
    /// # Arguments
    ///
    /// * `device_addr` - display to address as connected in series (0 -> last)
    /// * `bcd` - the bcd encoded string slice consisting of [0-9,-,E,L,H,P]
    /// where upper case input for alphabetic characters results in dot being set.
    /// Length of string is always 8 bytes, use spaces for blanking.
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned in case there was an error during data transfer
    ///
    /// # Examples
    ///
    /// ```
    /// let mut max7219 = MAX7219::from_spi_cs(1, spi, cs).unwrap();
    /// max7219.power_on();
    /// max7219.write_bcd(0, b"H_e_l_P?").unwrap();
    /// ```
    ///
    pub fn write_bcd(
        &mut self,
        device_addr: usize,
        bcd: &[u8; MAX_DIGITS],
    ) -> Result<(), DriverError> {
        self.set_decode_mode(0, DecodeMode::CodeBDigits7_0)?;

        let mut digit: u8 = MAX_DIGITS as u8;
        for b in bcd {
            self.c.write_raw(device_addr, digit, bcd_byte(*b))?;

            digit -= 1;
        }

        Ok(())
    }
    fn set_all_devices(
        &mut self,
        register_address: RegisterAddr,
        data: u8,
    ) -> Result<(), DriverError> {
        for i in 0..self.c.devices() {
            self.c.write_register_data(i, register_address, data)?;
        }
        Ok(())
    }
}

impl<SPI> MAX7219<SpiConnector<SPI>>
where
    SPI: Write<u8>,
{
    ///
    /// Construct a new MAX7219 driver instance from pre-existing SPI in full hardware mode.
    /// The SPI will control CS (LOAD) line according to it's internal mode set.
    /// If you need the CS line to be controlled manually use MAX7219::from_spi_cs
    ///
    /// * `NOTE` - make sure the SPI is initialized in MODE_0 with max 10 Mhz frequency.
    ///
    /// # Arguments
    ///
    /// * `displays` - number of displays connected in series
    /// * `spi` - the SPI interface initialized with MOSI, MISO(unused) and CLK
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned in case there was an error during data transfer
    ///
    pub fn from_spi(displays: usize, spi: SPI) -> Result<Self, DriverError> {
        MAX7219::new(SpiConnector::new(displays, spi))
    }
}

impl<SPI, CS> MAX7219<SpiConnectorCs<SPI, CS>>
where
    SPI: Write<u8>,
    CS: OutputPin,
{
    ///
    /// Construct a new MAX7219 driver instance from pre-existing SPI and CS pin
    /// set to output. This version of the connection uses the CS pin manually
    /// to avoid issues with how the CS mode is handled in hardware SPI implementations.
    ///
    /// * `NOTE` - make sure the SPI is initialized in MODE_0 with max 10 Mhz frequency.
    ///
    /// # Arguments
    ///
    /// * `displays` - number of displays connected in series
    /// * `spi` - the SPI interface initialized with MOSI, MISO(unused) and CLK
    /// * `cs` - the CS PIN used to LOAD register on the display set to output mode
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned in case there was an error during data transfer
    ///
    pub fn from_spi_cs(displays: usize, spi: SPI, cs: CS) -> Result<Self, DriverError> {
        MAX7219::new(SpiConnectorCs::new(displays, spi, cs))
    }
}

///
/// Translate alphanumeric ASCII bytes into BCD
/// encoded bytes expected by the display chip.
///
fn bcd_byte(b: u8) -> u8 {
    match b as char {
        ' ' => 0b0000_1111, // "blank"
        '-' => 0b0000_1010, // - without .
        'e' => 0b0000_1011, // E without .
        'E' => 0b1000_1011, // E with .
        'h' => 0b0000_1100, // H without .
        'H' => 0b1000_1100, // H with .
        'l' => 0b0000_1101, // L without .
        'L' => 0b1000_1101, // L with .
        'p' => 0b0000_1110, // L without .
        'P' => 0b1000_1110, // L with .
        _ => b,
    }
}

///
/// Translate alphanumeric ASCII bytes into segment set bytes
///
fn ssb_byte(b: u8, dot: bool) -> u8 {
    let mut result = match b as char {
        ' ' => 0b0000_0000, // "blank"
        '.' => 0b1000_0000,
        '-' => 0b0000_0001, // -
        '_' => 0b0000_1000, // _
        '0' => 0b0111_1110,
        '1' => 0b0011_0000,
        '2' => 0b0110_1101,
        '3' => 0b0111_1001,
        '4' => 0b0011_0011,
        '5' => 0b0101_1011,
        '6' => 0b0101_1111,
        '7' => 0b0111_0000,
        '8' => 0b0111_1111,
        '9' => 0b0111_1011,
        'a' | 'A' => 0b0111_0111,
        'b' => 0b0001_1111,
        'c' | 'C' => 0b0100_1110,
        'd' => 0b0011_1101,
        'e' | 'E' => 0b0100_1111,
        'f' | 'F' => 0b0100_0111,
        'g' | 'G' => 0b0101_1110,
        'h' | 'H' => 0b0011_0111,
        'i' | 'I' => 0b0011_0000,
        'j' | 'J' => 0b0011_1100,
        // K undoable
        'l' | 'L' => 0b0000_1110,
        // M undoable
        'n' | 'N' => 0b0001_0101,
        'o' | 'O' => 0b0111_1110,
        'p' | 'P' => 0b0110_0111,
        'q' => 0b0111_0011,
        // R undoable
        's' | 'S' => 0b0101_1011,
        // T undoable
        'u' | 'U' => 0b0011_1110,
        // V undoable
        // W undoable
        // X undoable
        // Y undoable
        // Z undoable
        _ => 0b1110_0101, // ?
    };

    if dot {
        result |= 0b1000_0000; // turn "." on
    }

    result
}

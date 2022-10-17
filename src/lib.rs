#![no_std]

pub mod enums;
use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v2::OutputPin;
use enums::*;

/// The MAX7219 Driver that initializes and communicates with the MAX7219 IC or chain of ICs.
pub struct MAX7219<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS> MAX7219<SPI, CS>
where
    SPI: Write<u8>,
    CS: OutputPin,
{
    ///
    /// Constructor method. Creates a new instance of the MAX7219 driver.
    ///
    pub fn new(spi: SPI, cs: CS) -> Result<Self, DriverError> {
        let max7219 = MAX7219 { spi, cs };
        Ok(max7219)
    }

    ///
    /// Transmits raw data to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `arr` - slice of data that needs to be transmitted
    ///
    /// # Errors
    ///
    /// * `DriverError` - returned if there is an error during transfer or addressing device
    ///
    pub fn transmit_raw_data(&mut self, arr: &[u8]) -> Result<(), DriverError> {
        self.cs.set_low().map_err(|_| DriverError::PinError)?;
        let transfer = self.spi.write(&arr).map_err(|_| DriverError::SpiError);
        self.cs.set_high().map_err(|_| DriverError::PinError)?;
        transfer
    }

    ///
    /// Configures the power mode of the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `Shutdown` enum
    ///
    pub fn config_power_mode(&mut self, mode: Power) -> () {
        let send_array: [u8; 2] = [Command::Shutdown as u8, mode as u8];
        // Transmit Data
        self.transmit_raw_data(&send_array).unwrap();
    }

    ///
    /// Configures the decode mode on the input sent to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `DecodeMode` enum
    ///
    pub fn config_decode_mode(&mut self, mode: DecodeMode) -> () {
        // Package into array to pass to SPI write method
        // Write method will grab array and send all data in it
        let send_array: [u8; 2] = [Command::DecodeMode as u8, mode as u8];
        // Transmit Data
        self.transmit_raw_data(&send_array).unwrap();
    }
    ///
    /// Configures the intensity of the LEDs on the display connected to the MAX7219 IC.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `Intensity` enum
    ///
    pub fn config_intensity(&mut self, mode: Intensity) -> () {
        // Package into array to pass to SPI write method
        // Write method will grab array and send all data in it
        let send_array: [u8; 2] = [Command::Intensity as u8, mode as u8];
        // Transmit Data
        self.transmit_raw_data(&send_array).unwrap();
    }

    ///
    /// Configures the scanlimit for the MAX7219 IC.
    /// Applicable mostly to seven segment displays if certain digits (ex. on the left)
    /// need not to be shown.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `ScanLimit` enum
    ///
    pub fn config_scan_limit(&mut self, mode: ScanLimit) -> () {
        // Package into array to pass to SPI write method
        // Write method will grab array and send all data in it
        let send_array: [u8; 2] = [Command::ScanLimit as u8, mode as u8];
        // Transmit Data
        self.transmit_raw_data(&send_array).unwrap();
    }

    ///
    /// Method to perform a visual test of the display.
    /// If performing a test, display needs to be put back in normal operation mode after done.
    ///
    /// # Arguments
    ///
    /// * `mode` - one of the options in the `DisplayTest` enum
    ///
    pub fn display_test(&mut self, mode: DisplayTest) -> () {
        // Package into array to pass to SPI write method
        // Write method will grab array and send all data in it
        let send_array: [u8; 2] = [Command::DisplayTest as u8, mode as u8];
        // Transmit Data
        self.transmit_raw_data(&send_array).unwrap();
    }

    ///
    /// Method to draw a row if the MAX7219 is driving an 8x8 LED dot matrix display.
    /// Alternatively method will draw a digit in case the MAX7219 is driving a seven-segment display
    ///
    /// # Arguments
    ///
    /// * `digit_addr` - one of the options in the `DigitRowAddress` enum
    /// * `led_data` - the led row or seven segment digit activation data
    ///
    pub fn draw_row_or_digit(&mut self, digit_addr: DigitRowAddress, led_data: u8) -> () {
        let send_array: [u8; 2] = [digit_addr as u8, led_data];
        self.transmit_raw_data(&send_array).unwrap();
        ()
    }

    ///
    /// Method to clear the display.
    ///
    pub fn clear_display(&mut self) -> () {
        for i in 1..9 {
            self.transmit_raw_data(&[i]).unwrap();
        }
    }

    ///
    /// Method to initialize the MAX7219 and the connected display.
    /// This method has to be called before doing any display operations otherwise the display will not operate properly.
    /// The method provides an option to leave the display uncleared after initalization.
    ///  
    /// # Arguments
    ///
    /// * `clr_display` - Boolean that reflects whether the display should be cleared or not after init
    ///
    pub fn init_display(&mut self, clr_display: bool) -> () {
        // 1.a) Power Up Device
        self.config_power_mode(Power::NormalOperation);
        // 1.b) Set up Decode Mode
        self.config_decode_mode(DecodeMode::Nodecode);
        // 1.c) Configure Scan Limit
        self.config_scan_limit(ScanLimit::Display0To7);
        // 1.d) Configure Intensity
        self.config_intensity(Intensity::Ratio15_32);
        // 1.e) Optional Screen Clear on Init
        if clr_display {
            self.clear_display();
        }
    }
}

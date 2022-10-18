//! //! MAX7219 Demo
//!
//! This example drives an wokwi-max7219-matrix Dot Matrix.
//! You can see the simulation by [`https://wokwi.com/projects/345754769993761364`]
//! Max7219 is connected to on the ESP32-C3 boards.

#![no_std]
#![no_main]

use esp32c3_hal::{
    clock::ClockControl,
    pac::Peripherals,
    prelude::*,
    spi::{Spi, SpiMode},
    timer::TimerGroup,
    Delay, Rtc, IO,
};
use esp_backtrace as _;
use esp_println::println;
use max7219_driver::{
    global::enums::{DisplayTest, Intensity},
    MAX7219,
};
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // GPIO2 (FSPIQ: MISO/SIO1)、
    // GPIO6 (FSPICLK)、GPIO7 (FSPID: MOSI/SIO0)、
    let sclk = io.pins.gpio6;
    let mosi = io.pins.gpio7;
    let cs = io.pins.gpio5.into_push_pull_output();

    let spi = Spi::new_no_cs_no_miso(
        peripherals.SPI2,
        sclk,
        mosi,
        800u32.Hz(),
        SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );

    let mut max7219 = MAX7219::from_spi_cs(2, spi, cs).unwrap();

    // Initialize the Delay peripheral
    let mut delay = Delay::new(&clocks);

    max7219.power_on().unwrap();

    println!("hello max7219!");
    let z = [
        0b0000_0000,
        0b01111110,
        0b00100000,
        0b00010000,
        0b00001000,
        0b00000100,
        0b01111110,
        0b00000000,
    ];
    let h = [
        0b00000000,
        0b01000010,
        0b01000010,
        0b01111110,
        0b01111110,
        0b01000010,
        0b01000010,
        0b0000_0000,
    ];
    max7219.set_display_test_mode_all(DisplayTest::DisplayTestMode);
    delay.delay_ms(2000_u32);

    loop {
        // test write_raw
        max7219.write_raw_all(0, &z).unwrap();
        max7219.write_raw_all(1, &h).unwrap();
        max7219.set_intensity(0, Intensity::Ratio25_32);
        max7219.set_intensity(1, Intensity::Ratio5_32);
    }
}

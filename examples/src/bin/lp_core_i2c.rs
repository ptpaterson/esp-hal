//! This shows a very basic example of running code on the LP core.
//!
//! Code on LP core uses LP_I2C initialized on HP core. For more information
//! check `lp_core_i2c` example in the `esp-lp-hal`.
//!
//! Make sure to first compile the `esp-lp-hal/examples/i2c.rs` example

//% CHIPS: esp32c6

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    gpio::lp_gpio::IntoLowPowerPin,
    i2c::lp_i2c::LpI2c,
    lp_core::{LpCore, LpCoreWakeupSource},
    peripherals::Peripherals,
    prelude::*,
    IO,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let lp_sda = io.pins.gpio6.into_low_power().into_open_drain_output();
    let lp_scl = io.pins.gpio7.into_low_power().into_open_drain_output();

    let lp_i2c = LpI2c::new(peripherals.LP_I2C0, lp_sda, lp_scl, 100u32.kHz());

    let mut lp_core = LpCore::new(peripherals.LP_CORE);
    lp_core.stop();
    println!("lp core stopped");

    // load code to LP core
    let lp_core_code =
        load_lp_code!("../esp-lp-hal/target/riscv32imac-unknown-none-elf/release/examples/i2c");

    // start LP core
    lp_core_code.run(&mut lp_core, LpCoreWakeupSource::HpCpu, lp_i2c);
    println!("lpcore run");

    loop {}
}

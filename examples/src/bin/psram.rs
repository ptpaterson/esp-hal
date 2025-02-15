//! This shows how to use PSRAM as heap-memory via esp-alloc
//!
//! You need an ESP32, ESP32-S2, or ESP32-S3 with at least 2 MB of PSRAM memory.

//% CHIPS: esp32 esp32s2 esp32s3
//% FEATURES: psram-2m

#![no_std]
#![no_main]

extern crate alloc;

use alloc::{string::String, vec::Vec};

use esp_backtrace as _;
use esp_hal::{
    clock::{ClockControl, CpuClock},
    peripherals::Peripherals,
    prelude::*,
    psram,
};
use esp_println::println;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_psram_heap() {
    unsafe {
        ALLOCATOR.init(psram::psram_vaddr_start() as *mut u8, psram::PSRAM_BYTES);
    }
}

#[entry]
fn main() -> ! {
    #[cfg(debug_assertions)]
    compile_error!("PSRAM example must be built in release mode!");

    let peripherals = Peripherals::take();

    psram::init_psram(peripherals.PSRAM);
    init_psram_heap();

    let system = peripherals.SYSTEM.split();
    let _clocks = ClockControl::max(system.clock_control).freeze();

    println!("Going to access PSRAM");
    let mut large_vec = Vec::<u32>::with_capacity(500 * 1024 / 4);

    for i in 0..(500 * 1024 / 4) {
        large_vec.push((i & 0xff) as u32);
    }

    println!("vec size = {} bytes", large_vec.len() * 4);
    println!("vec address = {:p}", large_vec.as_ptr());
    println!("vec[..100] = {:?}", &large_vec[..100]);

    let string = String::from("A string allocated in PSRAM");
    println!("'{}' allocated at {:p}", &string, string.as_ptr());

    println!("done");

    loop {}
}

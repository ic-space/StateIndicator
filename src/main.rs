#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use ics_state_indicator::state_model;
use panic_halt as _;
use rp2040_hal::{gpio::bank0::Gpio25, gpio::Pin, gpio::PushPullOutput};
/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[rp2040_hal::entry]
fn main() -> ! {
    let (mut delay, pins) = state_model::init();
    let mut pin25: Pin<Gpio25, PushPullOutput> = pins.gpio25.into_mode();
    loop {
        state_model::setup_blink(&mut delay, &mut pin25);
    }
}

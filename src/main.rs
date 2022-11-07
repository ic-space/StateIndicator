#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;
use state_indicator::init;
use rp2040_hal::gpio::PushPullOutput;
use rp2040_hal::gpio::bank0::Gpio25;
use rp2040_hal::gpio::Pin;
use embedded_hal::digital::v2::OutputPin;
/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;
/// Entry point to our bare-metal application.
///
/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
const DELAY_LEN: u32 = 50;
/// The function configures the RP2040 peripherals, then fades the LED in an
/// infinite loop.
#[rp2040_hal::entry]
fn main() -> ! {
    let (p, mut d) = init();
    let mut pin25: Pin<Gpio25, PushPullOutput> = p.gpio25.into_mode();
    loop { 
            pin25.set_high().unwrap();
            d.delay_ms(DELAY_LEN);
            pin25.set_low().unwrap();
            d.delay_ms(DELAY_LEN);
        }
}
#![no_std]

pub mod state_model {
    use cortex_m::delay::Delay;
    use embedded_hal::digital::v2::OutputPin;
    use rp2040_hal::{
        clocks::init_clocks_and_plls, gpio::bank0::Gpio25, gpio::Pin, gpio::Pins,
        gpio::PushPullOutput, pac, Clock, Sio, Watchdog,
    };

    /// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
    /// if your board has a different frequency
    const XTAL_FREQ_HZ: u32 = 12_000_000u32;

    const DELAY_LEN: u32 = 150;

    pub fn init() -> (Delay, Pins) {
        let mut peripherals = pac::Peripherals::take().unwrap();
        let core = pac::CorePeripherals::take().unwrap();
        // Set up the watchdog driver - needed by the clock setup code
        let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
        // The default is to generate a 125 MHz system clock
        let clocks = init_clocks_and_plls(
            XTAL_FREQ_HZ,
            peripherals.XOSC,
            peripherals.CLOCKS,
            peripherals.PLL_SYS,
            peripherals.PLL_USB,
            &mut peripherals.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let sio = Sio::new(peripherals.SIO);
        let pins = Pins::new(
            peripherals.IO_BANK0,
            peripherals.PADS_BANK0,
            sio.gpio_bank0,
            &mut peripherals.RESETS,
        );
        // The delay object lets us wait for specified amounts of time (in milliseconds)
        let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
        (delay, pins)
    }

    pub fn setup_blink<'a>(d: &'a mut Delay, p: &'a mut Pin<Gpio25, PushPullOutput>) {
        p.set_high().unwrap();
        d.delay_ms(DELAY_LEN);
        p.set_low().unwrap();
        d.delay_ms(DELAY_LEN);
    }
}

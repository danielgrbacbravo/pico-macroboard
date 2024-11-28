use cortex_m_rt::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rp_pico::hal::{self, gpio::*, pac};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut pins = hal::Pins::new(pac.GPIO, pac.IO_BANK0, pac.SIO);

    // Set up the encoder pins
    let enc_left = pins.gpio16.into_floating_input(); // ENC_LEFT
    let enc_right = pins.gpio17.into_floating_input(); // ENC_RIGHT
    let enc_btn = pins.gpio18.into_pull_up_input(); // ENC_BTN (with pull-up resistor)

    // Example for reading encoder pins (you can expand this logic to process rotary input)
    loop {
        if enc_left.is_high().unwrap() {
            // Do something when ENC_LEFT is high
        }
        if enc_right.is_high().unwrap() {
            // Do something when ENC_RIGHT is high
        }
        if enc_btn.is_low().unwrap() {
            // Do something when the button is pressed (low means button pressed)
        }
    }
}

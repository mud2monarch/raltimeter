#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_backtrace as _;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
};
use log::info;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

enum State {
    Blinking,
    PreSleep,
    DeepSleep,
}

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let mut peripherals = esp_hal::init(config);
    esp_println::logger::init_logger_from_env();

    let mut state = State::Blinking;

    info!("Hello world.");
    let mut led = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());
    let button = Input::new(
        peripherals.GPIO9,
        InputConfig::default().with_pull(Pull::Up),
    );
    let delay = Delay::new();

    loop {
        match state {
            State::Blinking => {
                info!("Now in Blinking...");
                for _ in 0..100 {
                    if button.is_low() {
                        delay.delay_millis(20);
                        if button.is_low() {
                            info!("Button pressed!");
                            state = State::PreSleep;
                        }
                        while button.is_low() {
                            delay.delay_millis(5);
                        }
                        continue;
                    }
                    delay.delay_millis(10);
                }
                led.toggle();
            }
            State::PreSleep => {
                info!("Now in PreSleep...");
                state = State::DeepSleep;
            }
            _ => {}
        }
    }
}

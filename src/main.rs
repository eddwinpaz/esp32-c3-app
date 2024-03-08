use embedded_graphics::pixelcolor::BinaryColor;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::{ i2c, peripherals::Peripherals };
use esp_idf_svc::hal::units::FromValueType;
use ssd1306::mode::DisplayConfig;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize72x40;
use ssd1306::{ I2CDisplayInterface, Ssd1306 };

use embedded_graphics::{ mono_font::{ ascii::FONT_6X10, MonoTextStyle }, prelude::*, text::Text };

fn main() {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let scl = peripherals.pins.gpio6;
    let sda = peripherals.pins.gpio5;

    let _cfg = i2c::config::Config::new().baudrate(FromValueType::kHz(400).into());
    let _i2c = i2c::I2cDriver::new(peripherals.i2c0, sda, scl, &_cfg).unwrap();
    let interface = I2CDisplayInterface::new(_i2c);

    let mut display = Ssd1306::new(
        interface,
        DisplaySize72x40,
        DisplayRotation::Rotate0
    ).into_buffered_graphics_mode();

    display.init().expect("Init Error");

    let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    loop {
        println!("Showing temperature");
        Text::new("Temperature\n36.5 C", Point::new(0, 9), style).draw(&mut display).unwrap();

        display.flush().unwrap();
        display.clear_buffer();
        FreeRtos::delay_ms(3000);

        println!("Showing humidity");
        Text::new("Humidity\n40.0", Point::new(0, 9), style).draw(&mut display).unwrap();
        display.flush().unwrap();
        display.clear_buffer();
        FreeRtos::delay_ms(3000);
    }
}

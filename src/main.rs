use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use esp_idf_svc::hal::{ i2c, peripherals::Peripherals };
use esp_idf_svc::hal::units::FromValueType;
use ssd1306::mode::DisplayConfig;
use ssd1306::rotation::DisplayRotation;
use ssd1306::size::DisplaySize128x32;
use ssd1306::{ I2CDisplayInterface, Ssd1306 };
use embedded_graphics::mono_font::ascii::FONT_6X10;

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
        DisplaySize128x32,
        DisplayRotation::Rotate0
    ).into_buffered_graphics_mode();

    display.init().expect("Init Error");

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    embedded_graphics::text::Text::new(
        "Hello Rust!",
        embedded_graphics::prelude::Point::new(0, 0),
        text_style
    );

    display.flush().unwrap();
}

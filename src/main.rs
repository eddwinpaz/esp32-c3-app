use std::error::Error;
use std::str::FromStr;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use esp_idf_svc::eventloop::EspSystemEventLoop;
// use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::{ peripherals::Peripherals, gpio::PinDriver };
use esp_idf_svc::http::{ Method::Get, server::EspHttpServer };
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{ ClientConfiguration, Configuration, EspWifi };
use heapless::String;
// use log::println;
// use std::env;

fn main() {
    esp_idf_svc::sys::link_patches();

    let ssid: String<32> = String::from_str("movistar2,4GHZ_EF39B0").unwrap();
    let password: String<64> = String::from_str("QFrjq9dwT8b4ZvyYBZ52").unwrap();

    let peripherals = Peripherals::take().unwrap();
    let led_pin = PinDriver::output(peripherals.pins.gpio1).expect(
        "Error: Unable to set pin(led_pin) gpio2 Output"
    );
    let led_pin = Mutex::new(led_pin);

    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi_driver = EspWifi::new(peripherals.modem, sys_loop, Some(nvs)).unwrap();

    wifi_driver
        .set_configuration(
            &Configuration::Client(ClientConfiguration {
                ssid,
                password,
                ..Default::default()
            })
        )
        .unwrap();

    wifi_driver.start().unwrap();
    println!("Wifi Started");

    wifi_driver.connect().unwrap();
    println!("Wifi connected");

    while !wifi_driver.is_connected().unwrap() {
        let config = wifi_driver.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }
    println!("Should be connected now");

    let mut server = EspHttpServer::new(&Default::default()).unwrap();

    server
        .fn_handler(
            "/led",
            Get,
            |_| -> Result<(), Box<dyn Error>> {
                let mut led_pin = led_pin.lock().unwrap();
                led_pin.toggle().expect("Error: Could not toggle pin(led_pin) gpio2");
                println!("/led endpoint has been requested");
                Ok(())
            }
        )
        .unwrap();

    loop {
        println!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());
        sleep(Duration::new(10, 0));
    }

    // loop {
    //     led_pin.set_high().expect("Error: Unable to set pin(led_pin) gpio2 high");
    //     println!("LED ON");
    //     FreeRtos::delay_ms(3000);

    //     led_pin.set_low().expect("Error: Unable to set pin(led_pin) gpio2 high");
    //     println!("LED OFF");
    //     FreeRtos::delay_ms(3000);
    // }
}

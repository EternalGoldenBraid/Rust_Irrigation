// See https://www.arduino.cc/en/Reference/PortManipulation for port mapping.


#![no_std]
#![no_main]

use panic_halt as _;
//extern crate panic_halt;
//extern crate panic_abort;
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;

use crate::moisture::{SensorUnit};

mod moisture;

//#[arduino_uno::entry]
fn main() -> ! {

    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(
        peripherals.PORTB,
        peripherals.PORTC,
        peripherals.PORTD,
    );

    // Boards serial write to console
    // screen /dev/tty/<your_tty_here> 57600
    // ls /dev/tty* | grep usb --> get the usb connected
    // 57600 is the baud rate
    let mut serial = arduino_uno::Serial::new(
        peripherals.USART0,

        // the values below correspond to :
        // rx: receive pin (hardwired into the MCU)
        // tx : PD1 is the "hardcoded output"
        // the ownership is moved by writing explicitely input, 
        // output is enforced at compile time,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),

        // other well known baud rates are possible (9600)
        //57600,
        57600.into_baudrate(),
        );

    // Read from moisture sensor.

    let mut sensor_unit = SensorUnit 
    {
        a0in: pins.a0.into_floating_input(&mut pins.ddr),
    };


    loop {
        ufmt::uwriteln!( serial, "asd" ).void_unwrap();

    }

    //loop {
    //    //let value = sensor_unit.a0in.read().bits();
    //    if sensor_unit.a0in.is_high().void_unwrap() {
    //        ufmt::uwriteln!( 
    //            &mut serial,
    //            "1.\r"
    //        ).void_unwrap();
    //    } else {
    //        ufmt::uwriteln!( 
    //            &mut serial,
    //            "0.\r"
    //        ).void_unwrap();
    //    }
    //}

}

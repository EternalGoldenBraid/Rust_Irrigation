// See https://www.arduino.cc/en/Reference/PortManipulation for port mapping.


#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use panic_halt as _;
//extern crate panic_abort;

// The prelude just exports all HAL traits anonymously which makes
// all trait methods available.  This is probably something that
// should always be added.
use arduino_uno::prelude::*;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::adc;
//use ufmt::{uwriteln};
//use ufmt_float::uFmt_f32;

use crate::moisture::{SensorUnit};

mod moisture;

const DELAY: u16 = 1000u16;
//const WAIT_WATER: u16 = 60000u16; // 1 minute
const WAIT_WATER: u16 = 1000u16; // 1 minute
const DRY:   u16 = 464u16;
const WET:   u16 = 210u16;
const THRESHOLD_MEASUREMENTS: u16 = 10;

// Define the entry-point for the application.  This can only be
// done once in the entire dependency tree.
#[arduino_uno::entry]
fn main() -> ! {


    // Get the peripheral singletons for interacting with them.
    //let dp = arduino_uno::Peripherals::take().unwrap();
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
        //57600.into_baudrate(),
        9600.into_baudrate(),
        );

    



    //let mut led = pins.d13.into_output(&mut pins.ddr);
    //led.set_high().void_unwrap();

    // Read moisture output; WRITE THIS INTO IT'S OWN FUNCTION IN THE MOISTURE MODULE.
    let mut adc = adc::Adc::new(peripherals.ADC, Default::default());
    let (vbg, gnd): (u16, u16) = (
        nb::block!(adc.read(&mut adc::channel::Vbg)).void_unwrap(),
        nb::block!(adc.read(&mut adc::channel::Gnd)).void_unwrap(),
    );
    
    // print reference voltage?
    //ufmt::uwriteln!(&mut serial, "Vbandgap: {}\r", vbg).void_unwrap();
    //ufmt::uwriteln!(&mut serial, "GND: {}\r", gnd).void_unwrap();
    
    let mut a0 = pins.a0.into_analog_input(&mut adc);

    let mut n = 0;
    let mut total = 0;
    let mut avg = 0;
    loop {
        
        let mut value: u16 = nb::block!(adc.read(&mut a0)).void_unwrap();
        total = total + value;

        n += 1;
        ufmt::uwrite!(&mut serial, "{}.: {}", n, value).void_unwrap();

        // Attempted conversion to float but returns zero.
        //let mut value_float = value as f32;
        //let coef = 3.3/1023.0;
        //value_float = value_float*coef;
        //let value_float = uFmt_f32::Four(value_float.into());
        //ufmt::uwriteln!(&mut serial, "{}", value_float).void_unwrap();
        
        //if n >= THRESHOLD_MEASUREMENTS;
        //let values: [u16; 1] = [
        //    nb::block!(adc.read(&mut a0)).void_unwrap(),
        //];

        //for (i, v) in values.iter().enumerate() {
        //    ufmt::uwrite!(&mut serial, "A{}: {} ", i, v*3.3/1023.0).void_unwrap();
        //}
          
        ufmt::uwriteln!(&mut serial, "\r").void_unwrap(); //newline
          

        if n >= THRESHOLD_MEASUREMENTS {
            avg = total/n;
            ufmt::uwriteln!(&mut serial, "Average is: {}", avg).void_unwrap();
            ufmt::uwriteln!(&mut serial, "\r").void_unwrap(); //newline
            n = 0;
            total = 0;
            avg = 0

        }
        arduino_uno::delay_ms(DELAY);
        
        // Display testing
        // SDA is connected to a4 and CLK to A5.

        //let mut data = pins.a4.into_analog_input(&mut adc);
        //let mut data: u8 = 0u8;
        let data = arduino_uno::hal::port::portc::PC4<arduino_uno::hal::port::mode::Output>;
        let clk = arduino_uno::hal::port::portc::PC5<arduino_uno::hal::port::mode::Output>;

        // Chip select (active LOW).


    }
}

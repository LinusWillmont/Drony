#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use arduino_hal::simple_pwm::IntoPwmPin;
use arduino_hal::simple_pwm::{Prescaler, Timer0Pwm};
use embedded_hal::digital::OutputPin;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let mut _onboard_led = pins.d13.into_output();

    let x = pins.a1.into_analog_input(&mut adc);
    let y = pins.a2.into_analog_input(&mut adc);

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let mut pwm = pins.d6.into_output().into_pwm(&timer0);
    pwm.enable();

    let mut direction_pin = pins.d5.into_output();
    direction_pin.set_high();

    loop {
        let x_voltage = x.analog_read(&mut adc);
        let y_voltage = y.analog_read(&mut adc);
        ufmt::uwriteln!(&mut serial, "x: {}, y: {}", x_voltage, y_voltage).unwrap();

        let normalized_y = y_voltage / (1024 / 255);

        let duty = normalized_y as u8;
        ufmt::uwriteln!(&mut serial, "duty: {}", duty).unwrap();

        pwm.set_duty(duty);

        delay_ms(30);
    }
}

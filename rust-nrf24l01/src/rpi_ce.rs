extern crate rppal;

use std::io;

use rppal::gpio::OutputPin;

use self::rppal::gpio::{Gpio as GPIO, Level, Mode};

pub struct CEPin {
    gpio: OutputPin,
    ce_pin: u8,
    value: Level,
    saved_value: Level,
}

impl CEPin {
    // add code here
    pub fn new(pin_num: u64) -> io::Result<CEPin> {
        let pin_num8 = pin_num as u8;
        let mut gpio = GPIO::new().unwrap();

        let mut pin = gpio.get(pin_num8).unwrap().into_output();

        Ok(CEPin {
            gpio: pin,
            ce_pin: pin_num8,
            value: Level::Low,
            saved_value: Level::Low,
        })
    }

    pub fn up(&mut self) -> io::Result<()> {
        self.gpio.set_high();
        self.value = Level::High;
        Ok(())
    }

    pub fn down(&mut self) -> io::Result<()> {
        self.gpio.set_low();
        self.value = Level::Low;
        Ok(())
    }

    pub fn save_state(&mut self) -> () {
        self.saved_value = self.value;
    }

    pub fn restore_state(&mut self) -> io::Result<()> {
        match self.saved_value {
            Level::High => self.gpio.set_high(),
            Level::Low => self.gpio.set_low(),
        }

        self.value = self.saved_value;
        Ok(())
    }
}

use std::io::Write;

use cpu6502::device::Device;

const START_ADDRESS: u16 = 0x8000;
const END_ADDRESS: u16 = 0x8002;

// Global variable for the LED strip
static mut LED_STRIP: [bool; 8] = [false; 8];

pub struct LedBlink {
    pub write_enabled: bool,
    start_address: u16,
    end_address: u16,
    address: u16,
    data: u8,

    // Local variable for the LED strip state
    enabled: bool,
}

impl LedBlink {
    pub fn new() -> Self {
        Self {
            write_enabled: true,
            start_address: START_ADDRESS,
            end_address: END_ADDRESS,
            address: 0,
            data: 0,
            enabled: false,
        }
    }

    pub fn update(&self) {
        unsafe {
            for i in 0..8 {
                LED_STRIP[i] = (self.data & (1 << i)) != 0;
            }
        }

        // Clear the line and print the LED strip
        print!("\x1B[K");
        print!("\rLED STRIP: ");
        for i in 0..8 {
            if unsafe { LED_STRIP[i] } {
                print!("█");
            } else {
                print!("░");
            }
        }

        // Flush stdout
        std::io::stdout().flush().unwrap();
    }
}

impl Device for LedBlink {
    fn start_address(&self) -> u16 {
        self.start_address
    }

    fn end_address(&self) -> u16 {
        self.end_address
    }

    fn set_address_data(&mut self, address: u16, data: u8) {
        self.address = address;
        self.data = data;
    }

    fn read_address_data(&self) -> (u16, u8) {
        (self.address, self.data)
    }

    fn set_write_enable(&mut self, write_enable: bool) {
        self.write_enabled = write_enable;
    }

    fn read(&mut self) {
        // This device can't be read from
    }

    fn write(&mut self, _force_write: bool) {
        // If we wrote 0xFF to address 8002, enable the LED strip
        if (self.address == END_ADDRESS) && (self.data == 0xFF) {
            self.enabled = true;
        }
        
        // If the LED strip is enabled, update it
        if self.enabled {
            self.update();
        }
    }
}
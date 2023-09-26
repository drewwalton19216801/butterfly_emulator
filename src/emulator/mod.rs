use std::ops::Range;
use std::{cell::RefCell, rc::Rc};

use cpu6502::Cpu;
use cpu6502::bus::Hook;

// Global variable for the LED strip
static mut LED_STRIP: [bool; 8] = [false; 8];
pub struct Emulator {
    pub cpu: Cpu,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
        }
    }

    pub fn run(&mut self) {
        // Register a write hook
        self.cpu.bus.add_hook_range(
            Range {
                start: 0x6000,
                end: 0x6002,
            },
            Hook {
                read: None,
                write: Some(Rc::new(RefCell::new(Self::blink_led))),
            }
        );

        // Change variant to CMOS
        self.cpu.change_variant(cpu6502::Variant::CMOS);

        // Reset the cpu
        self.cpu.reset();

        // Clock the CPU for 100 cycles
        for _ in 0..100 {
            self.cpu.clock();
        }
    }

    pub fn change_variant(&mut self, variant: String) {
        match variant.as_str() {
            "NMOS" => self.cpu.change_variant(cpu6502::Variant::NMOS),
            "CMOS" => self.cpu.change_variant(cpu6502::Variant::CMOS),
            "NES" => self.cpu.change_variant(cpu6502::Variant::NES),
            _ => panic!("Invalid CPU variant"),
        }
    }

    pub fn load_rom_from_path(&mut self, path: &str, address: u16) {
        let rom = std::fs::read(path).unwrap();
        self.cpu.bus.load_rom_at(&rom, address);
    }

    pub fn blink_led(address: u16, data: u8) {
        // If the address is 0x6002 and the data is 0xFF, then we want to turn on the LED
        if address == 0x6002 && data == 0xFF {
            unsafe {
                LED_STRIP[0] = true;
            }
        }

        // If the address is 0x6000, we want to enable the LED bits according to the data
        if address == 0x6000 {
            unsafe {
                for i in 0..8 {
                    LED_STRIP[i] = (data & (1 << i)) != 0;
                }
            }
        }

        // Print the LED strip
        unsafe {
            for i in 0..8 {
                print!("{}", if LED_STRIP[i] { "█" } else { " " });
            }
            println!();
        }
    }
}

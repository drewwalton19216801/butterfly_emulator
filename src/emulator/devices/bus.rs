
use cpu6502::{device::Device, bus::Bus};

/**
 * This is the main bus for the emulator. It is responsible for connecting all the devices together,
 * as well as providing the interface for the CPU to read and write to the devices.
 */

pub struct MainBus {
    devices: Vec<Box<dyn Device>>,
}

impl MainBus {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }
}

impl Bus for MainBus {
    fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device);
    }

    fn read(&mut self, address: u16) -> u8 {
        for device in self.devices.iter_mut() {
            if address >= device.start_address() && address <= device.end_address() {
                device.set_address_data(address, 0);
                device.read();
                let data = device.read_address_data().1;
                return data;
            }
        }
        0
    }

    fn write(&mut self, address: u16, data: u8) {
        for device in self.devices.iter_mut() {
            if address >= device.start_address() && address <= device.end_address() {
                device.set_address_data(address, data);
                device.write(true);
            }
        }
    }

    fn load_rom_at(&mut self, rom: &[u8], address: u16) {
        for (i, byte) in rom.iter().enumerate() {
            self.write(address + i as u16, *byte);
        }
    }
}
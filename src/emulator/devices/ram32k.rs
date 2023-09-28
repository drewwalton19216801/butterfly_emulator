use cpu6502::device::Device;

// 32KB of RAM
const START_ADDRESS: u16 = 0x0000;
const END_ADDRESS: u16 = 0x7FFF;

pub struct Ram32k {
    pub write_enabled: bool,
    start_address: u16,
    end_address: u16,
    address: u16,
    data: u8,

    // We have 64KB of memory space
    memory: Box<[u8; 0x10000]>,
}

impl Ram32k {
    pub fn new() -> Self {
        Self {
            write_enabled: true,
            start_address: START_ADDRESS,
            end_address: END_ADDRESS,
            address: 0,
            data: 0,
            memory: Box::new([0; 0x10000]),
        }
    }
}

impl Device for Ram32k {
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
        if (self.address >= START_ADDRESS) && (self.address <= END_ADDRESS) {
            // Grab the data from the ROM and set it to the data lines
            self.data = self.memory[self.address as usize];
        }
    }

    fn write(&mut self, force_write: bool) {
        // If the address is in the range of the ROM, then we want to write to it
        if (self.address >= START_ADDRESS) && (self.address <= END_ADDRESS) {
            // If the write enable line is high, then we want to write to the ROM
            if self.write_enabled || force_write {
                // Write the data to the ROM
                self.memory[self.address as usize] = self.data;
            }
        }
    }
}
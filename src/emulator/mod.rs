use cpu6502::Cpu;

use crate::emulator::devices::rom::Rom;

use self::devices::bus::MainBus;

pub mod devices;
pub struct Emulator {
    pub cpu: Cpu,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(Box::new(MainBus::new())),
        }
    }

    pub fn run(&mut self, speed_mhz: f64, num_cycles: Option<u64>) {
        let mut cycles_left = num_cycles.unwrap_or(u64::MAX);

        // Calculate the number of cycles to run per second
        let cycles_per_second = speed_mhz * 1_000_000.0;

        let mut disassembly_displayed = false;

        // Clock the CPU until we run out of cycles
        while cycles_left > 0 {
            if !disassembly_displayed {
                // Print the disassembly string
                println!("{}", self.cpu.current_instruction_string);
                // Set the flag
                disassembly_displayed = true;
            }
            self.cpu.clock();
            cycles_left -= 1;
            std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / cycles_per_second));
        }
    }

    pub fn benchmark(&mut self, rom_path: String, address: u16, variant: String) {
        self.cpu.bus.add_device(Box::new(Rom::new()));

        self.load_rom_from_path(&rom_path, address);

        self.cpu.reset();
        self.change_variant(variant);

        println!("Running CPU benchmark...");
        let num_cycles = 200_000_000;

        println!("Running for {} cycles...", num_cycles);
        // Start a timer
        let start = std::time::Instant::now();

        for _ in 0..num_cycles {
            self.cpu.clock();
        }

        let end = std::time::Instant::now();
        let time_elapsed = end.duration_since(start);
        println!("Benchmark complete. Calculating results...");

        let cycles_per_second = num_cycles as f64 / time_elapsed.as_secs_f64();

        // This is VERY generic, but it's reasonale to assume that the CPU will execute, on average,
        // 6 cycles per instruction
        let instructions_per_second = cycles_per_second / 6.0;

        let calculated_speed_mhz = cycles_per_second / 1_000_000.0;

        // Print the results
        println!("Cycles per second: {}", cycles_per_second);
        println!(
            "Average instructions per second*: {}",
            instructions_per_second
        );
        println!("Time elapsed: {:?}", time_elapsed);
        println!("MHz: {}", calculated_speed_mhz);
        println!("");
        println!("* This is the average number of instructions per second, as not all instructions take the same number of cycles.");
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
        println!("Loading ROM from path: {}", path);
        let rom = std::fs::read(path).unwrap();
        // MainBus::load_rom_at(&mut self.bus, &rom, address);
        self.cpu.bus.load_rom_at(&rom, address);
    }
}

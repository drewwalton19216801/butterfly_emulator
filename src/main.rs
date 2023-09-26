use std::env;

use emulator::{Emulator, devices::{led_blink::LedBlink, rom::Rom}};

mod emulator;
fn main() {
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();
    let (rom_path, address, variant, speed, benchmark_mode) = parse_args(args);

    // Create the emulator
    let mut emulator = Emulator::new();

    emulator
        .cpu
        .bus
        .add_device(Box::new(LedBlink::new()));

    emulator
        .cpu
        .bus
        .add_device(Box::new(Rom::new()));

    // Load the ROM file
    emulator.load_rom_from_path(&rom_path, address);

    // Reset the CPU
    emulator.cpu.reset();

    // Change the variant of the CPU
    emulator.change_variant(variant);

    // If benchmark mode is enabled, run the benchmark
    if benchmark_mode {
        emulator.benchmark();
        std::process::exit(0);
    }

    // Run the emulator
    emulator.run(speed, None);

    /*
    let mut emulator = emulator::emulator::Emulator::new();
    emulator.load_file_from_path("demos/blink.bin");
    emulator.run();
    */
}

fn parse_args(args: Vec<String>) -> (String, u16, String, f64, bool) {
    // Set the default values
    let mut rom_path = String::from("demos/blink.bin");
    let mut address = 0xC000;
    let mut variant = String::from("CMOS");
    let mut speed = 0.000100; // 100 Hz
    let mut benchmark_mode = false;

    // Parse the arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-r" | "--rom" => {
                rom_path = args[i + 1].clone();
                i += 1;
            }
            "-a" | "--address" => {
                address = u16::from_str_radix(&args[i + 1], 16).unwrap();
                i += 1;
            }
            "-v" | "--variant" => {
                variant = args[i + 1].clone();
                i += 1;
            }
            "-s" | "--speed" => {
                speed = args[i + 1].parse::<f64>().unwrap();
                i += 1;
            }
            "-b" | "--benchmark" => {
                benchmark_mode = true;
            }
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                println!("Invalid argument: {}", args[i]);
                print_help();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    // Return the parsed arguments
    (rom_path, address, variant, speed, benchmark_mode)
}

fn print_help() {
    println!("Usage: emulator [OPTIONS]");
    println!("Options:");
    println!("  -r, --rom: The path to the ROM file to load");
    println!("  -a, --address: The address to load the ROM at (default: 0xC000)");
    println!("  -v, --variant: The variant of the CPU to use");
    println!("     - NMOS: The MMOS 6502 CPU");
    println!("     - CMOS: The CMOS 65C02 CPU (default)");
    println!("     - NES: The NES CPU (Ricoh 2A03)");
    println!(
        "  -b, --benchmark: Runs demos/blink.bin for 200,000,000 cycles and prints the results"
    );
    println!("  -h, --help: Prints the help message");
}

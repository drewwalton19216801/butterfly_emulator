# Butterfly 6502-based emulator
## What is this?
It's an emulator designed to emulate a 6502 CPU and various peripherals.
## Why is this?
Because I thought it would be fun, I was also inspired by [Ben Eater's excellent 6502 video series](https://eater.net/6502). Originally I had wanted to create a breadboard computer, but realized that I didn't have the extra cash for things like logic analyzers or oscilloscopes. Besides, emulating in software gives me flexibility to create whatever hardware I want!
## Building
Build like you would any other Rust project:

    cargo build

## Running
A modified version of Ben Eater's blink.asm is provided in demos/blink.asm, and will automatically run for 1000 cycles by simply running the following:

    cargo run
### Benchmarking
By running the following command, you can run a very very unsophisticated benchmark.

    cargo run -- --benchmark
### Command-line options

    Usage: butterfly_emulator [OPTIONS]
    Options:
      -r, --rom: The path to the ROM file to load
      -a, --address: The address to load the ROM at (default: 0xC000)
      -v, --variant: The variant of the CPU to use
         - NMOS: The NMOS 6502 CPU
         - CMOS: The CMOS 65C02 CPU
         - NES: The NES CPU (Ricoh 2A03)
       -b, --benchmark: Runs demos/blink.bin for 200,000,000 cycles and prints the results
       -h, --help: Prints the help message

## Helpful Links
[NesDev CPU wiki](https://www.nesdev.org/wiki/CPU) - Fantastic resource for 6502 information, specifically the NES version of the 6502.

[mass:werk 6502 tools](https://www.masswerk.at/6502/) - A fully functional 6502 CPU emulator, assembler, and disassembler, as well as a great resource for 6502 instruction set internals
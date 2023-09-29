# 6502 Demo programs
## blink.asm
blink.asm is Ben Eater's LED blinking program from his [6502 video series](https://eater.net/6502), but has been modified to start at address $C000 instead of $8000.

## multiply10-blink.asm
multiply10loop.asm is a simple program that multiplys by 10 in a loop, starting with decimal number 1. It's based on [Leo Nechaev's fast x10 multiply code.](http://www.6502.org/source/integers/fastx10.htm),
but has been modified to output the multiplication results to the 8-bit LED strip used by blink.asm. When the counter reaches 0, it will enable all LEDs.

## multiply10-noblink.asm
multiply10loop-noblink.asm is the same thing as multiply10-blink.asm, but has been slightly modified to not output to the LED strip (in debug mode, the LED strip output mangles the debugging output. This will be fixed at some point).

## decimal-test.asm
decimal-test.asm is a program that counts from 0 to 50 in *decimal mode*, then counts back to 0, then back to 50, and so on, in an infinite loop. It outputs to the LED strip.

## Building
### blink.asm
To build the blink demo, you can use the following command from the root of the project:

    ./demos/utils/vasm6592_oldstyle.exe -Fbin -dotdir ./demos/blink.asm -o ./demos/blink.bin

### multiply10-blink.asm
To build the multiply10loop demo, you can use the following command from the root of the project:

    ./demos/utils/vasm6592_oldstyle.exe -Fbin -dotdir ./demos/multiply10-blink.asm -o ./demos/multiply10-blink.bin

### multiply10loop-noblink.asm
To build the multiply10loop-noblink demo, you can use the following command from the root of the project:

    ./demos/utils/vasm6592_oldstyle.exe -Fbin -dotdir ./demos/multiply10loop-noblink.asm -o ./demos/multiply10loop-noblink.bin

### decimal-test.asm
To build the decimal-test demo, you can use the following command from the root of the project:

    ./demos/utils/vasm6592_oldstyle.exe -Fbin -dotdir ./demos/decimal-test.asm -o ./demos/decimal-test.bin


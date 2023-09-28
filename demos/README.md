# 6502 Demo programs
## blink.asm
blink.asm is Ben Eater's LED blinking program from his [6502 video series](https://eater.net/6502), but has been modified to start at address $C000 instead of $8000.

## multiply10loop.asm
multiply10loop.asm is a simple program that multiplys by 10 in a loop, starting with decimal number 1. It's based on [Leo Nechaev's fast x10 multiply code.](http://www.6502.org/source/integers/fastx10.htm).
  

## Building
To build the blink demo, you can use the following command from the root of the project:

    ./demos/utils/vasm6592_oldstyle.exe -Fbin -dotdir ./demos/blink.asm -o ./demos/blink.bin

To build the multiply10loop demo, you can use the following command from the root of the project:

    ./demos/utils/vasm6592_oldstyle.exe -Fbin -dotdir ./demos/multiply10loop.asm -o ./demos/multiply10loop.bin


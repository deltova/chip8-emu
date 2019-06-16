# Chip8 Disassembler/Emulator 

This repository contains two programms:
- A Chip8 Disassembler
- A Chip8 Emulator

The main purpose of this project is to discover the Rust Programming Language

## Getting Started

To begin with this project you just could clone it from github.

## Disassembler

to disassemble a chip8 programm:
```
cd disassembler
cargo run <chip8-rom>
```

## Emulator
To run a chip8 rom:
```
cd emulator 
cargo run <chip8-rom>
```
For the moment the default scale of the screen is 3.
I will add a way to control this scale via the command line.

The emulator does not have full support of a real Chip8 machine.
Missing feature:
- Timer
- Keyboard inputs
- Sound

## Authors

* **Clement Magnard** - [Deltova](https://github.com/deltova)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

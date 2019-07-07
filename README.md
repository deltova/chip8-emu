# Chip8 Disassembler/Emulator 

This repository contains two programms:
- A Chip8 Disassembler
- A Chip8 Emulator

The main purpose of this project is to discover the Rust Programming Language

## Getting Started

To begin with this project you just could clone it from github.

## Disassembler

To disassemble a chip8 program:
```
cd disassembler
cargo run <chip8-rom>
```

## Emulator
#### How to use the emulator
```
USAGE:
    emulator [FLAGS] [OPTIONS] <rom>

FLAGS:
    -d, --debug      Turn debugging information on
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --scale <SCALE>    Set scale size for the screen

ARGS:
    <rom>    Input rom for the emulator
```

### Features
The emulator does not have full support of a real Chip8 machine.
Missing features:
- Timer
- Keyboard inputs
- Sound

## Authors

* **Clement Magnard** - [Deltova](https://github.com/deltova)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

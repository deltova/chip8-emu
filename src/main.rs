use std::env;
use std::fs;
use std::io::{Error, ErrorKind};

const SIZE_INSTR: usize = 2;


fn print_instr(instruction: u16) -> Result<(), std::io::Error> {
    let instr_tuple = ((instruction & 0xF000)>> 12, (instruction & 0xF00) >> 8,
                       (instruction & 0xF0) >> 4, instruction & 0xF);
    print!("0x{:X}    ", instruction);
    match instr_tuple {
        (0, b, c, d) => match (b, c, d) {
                (0, 0xE, 0) => println!("clear screen"),
                (0, 0xE, 0xE) => println!("return"),
                (b, c, d) => println!("call @0x{:X}{:X}{:X}", b, c, d),
            }
        (1, b, c, d) => println!("jmp 0x{:X}{:X}{:X}", b, c, d),
        (2, b, c, d) => println!("call 0x{:X}{:X}{:X}", b, c, d),
        (3, b, c, d) => println!("V{} == {}{}", b, c, d),
        (4, b, c, d) => println!("V{} != {}{}", b, c, d),
        (5, b, c, 0) => println!("V{} != V{}", b, c),
        (6, b, c, d) => println!("V{} = {}{}", b, c, d),
        (7, b, c, d) => println!("V{} += {}{}", b, c, d),
        (8, b, c, d) => match (b, c, d) {
            (b, c, 0) => println!("V{} = V{}", b, c),
            (b, c, 1) => println!("V{} |= V{}", b, c),
            (b, c, 2) => println!("V{} &= V{}", b, c),
            (b, c, 3) => println!("V{} ^= V{}", b, c),
            (b, c, 4) => println!("V{} += V{}", b, c),
            (b, c, 5) => println!("V{} -= V{}", b, c),
            (b, c, 6) => println!("V{} >>= 1", b),
            (b, c, 7) => println!("V{} = V{} - V{}", b, c, b),
            (b, c, E) => println!("V{} <<= 1", b),
            }
        (9, b, c, 0) => println!("V{} != V{}", b, c),
        (0xA, b, c, d) => println!("I = 0x{:X}{:X}{:X}", b, c, d),
        (0xB , b, c, d) => println!("PC = VO + 0x{:X}{:X}{:X}", b, c, d),
        (0xC , b, c, d) => println!("V{}=rand()&0x{:X}{:X} ", b, c, d),
        (0xD , b, c, d) => println!("draw(V{},V{}, {})", b, c, d),
        (0xE , b, 9, 0xE) => println!("if key() == V{}", b),
        (0xE , b, 0xA, 1) => println!("if key() != V{}", b),
        (0xF , b, 0, 7) => println!("V{} = get_delay()", b),
        (0xF , b, 0, 0xA) => println!("V{} = get_key()", b),
        (0xF , b, 1, 5) => println!("delay_timer(V{})", b),
        (0xF , b, 1, 8) => println!("sound_timer(V{})", b),
        (0xF , b, 2, 9) => println!("I = sprite_addr[V{}])", b),
        (0xF , b, 3, 3) => println!("BCD(V{})", b),
        (0xF , b, 5, 5) => println!("reg_dump(V{}, &I)", b),
        (0xF , b, 6, 5) => println!("reg_load(V{}, &I)", b),
        _ => print!("\n"),
    }
    Ok(())
}

fn open_and_read(file_name: &String) -> Result<(), std::io::Error> {
    let data = fs::read(file_name).expect("Unable to read file");
    for i in (1..data.len()).step_by(SIZE_INSTR) {
        let instruction : u16 = (data[i - 1] as u16) << 8 | (data[i] as u16); 
        print_instr(instruction);
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("too many or not enough args provided");
        Err(Error::new(ErrorKind::Other, "perdu"))
    }
    else {
        return open_and_read(&args[1]);
    }
}

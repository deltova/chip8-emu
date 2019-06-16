mod machine;
use machine::Machine;
use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::process;
use rand::Rng;
use ctrlc;

fn main() -> Result<(), std::io::Error> {
    // exit if ctrl-c is received
    ctrlc::set_handler(move || {
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("too many or not enough args provided");
        return Err(Error::new(ErrorKind::Other, "perdu"))
    }
    else {
        let mut machine : Machine = Machine::default();
        let instructions = fs::read(&args[1]).expect("Unable to read file");
        for (i, instr) in instructions.iter().enumerate() {
            machine.write_mem((512 + i) as u16, *instr);
        }
        while true {
            let pc = machine.pc();
            let first_byte = machine.read_mem(pc as u16);
            let sec_byte = machine.read_mem((pc + 1) as u16);
            let instruction : u16 = (first_byte as u16) << 8 | (sec_byte as u16); 
            dispatch_interpretor(instruction, &mut machine);
        }
    }
    return Ok(())
}

fn dispatch_interpretor(instruction: u16, machine: &mut Machine) {
    let instr_tuple = ((instruction & 0xF000)>> 12, (instruction & 0xF00) >> 8,
                       (instruction & 0xF0) >> 4, instruction & 0xF);
    match instr_tuple {
        (0, b, c, d) => match (b, c, d) {
                (0, 0xE, 0) => machine.clear_screen(),
                (0, 0xE, 0xE) => machine.returner(),
                (b, c, d) => println!("instruction call RCA implemented"),
            }
        (1, b, c, d) => {
                            machine.set_pc((b << 8 | c << 4 | d) as u16);
                            return;
                        },
        (2, b, c, d) => {
                            machine.call((b << 8 | c << 4 | d) as u16);
                            return;
                        },

        (3 ... 5, b, c, d) => condition(instr_tuple, machine), 
        // set reg
        (6, b, c, d) => machine.set_reg(b as usize, (c << 4 | d) as u8),
        // += reg
        (7, b, c, d) => machine.set_reg(b as usize,
                                        machine.get_reg(b as usize) + (c << 4 | d) as u8),
        (8, b, c, d) => match (b, c, d) {
                (b, c, 0 ... 7) => setter_regs(instr_tuple, machine),
                (b, c, e) => setter_regs(instr_tuple, machine),
                _  => println!("instruction not implemented"),
            }
        (9, b, c, 0) => condition(instr_tuple, machine),
        (0xA, b, c, d) => machine.set_i((b << 8| c << 4 | d) as u16),
        (0xB , b, c, d) => machine.set_pc((b << 8| c << 4 | d) as u16),
        (0xC , b, c, d) => random_instr(instr_tuple, machine),
        (0xD , b, c, d) => machine.draw(b as u8, c as u8, d as u8),
        (0xE , b, 9, 0xE) => println!("instruction key eq implemented"),
        (0xE , b, 0xA, 1) => println!("instruction key diff implemented"),
        (0xF , b, 0, 7) => println!("instruction get delay implemented"),
        (0xF , b, 0, 0xA) => println!("instruction get_key implemented"),
        (0xF , b, 1, 5) => println!("instruction timer implemented"),
        (0xF , b, 1, 8) => println!("instruction sound implemented"),
        (0xF , b, 1, 0xE) => machine.set_i(machine.get_i() + machine.get_reg(b as usize) as u16),
        (0xF , b, 2, 9) => println!("instruction load sprite implemented"),
        (0xF , b, 3, 3) => println!("instruction BCD implemented"),
        (0xF , b, 5, 5) => reg_dump(b as u8, machine),
        (0xF , b, 6, 5) => reg_load(b as u8, machine),
        (a, b, c, d) => process::exit(0),
    }
    machine.incr_pc();
}

fn reg_dump(reg_number: u8, machine: &mut Machine) {
    let base_addr = machine.get_i();
    for i in 0..reg_number + 1{
        machine.write_mem(base_addr + i as u16, machine.get_reg(i as usize));
    }

}

fn reg_load(reg_number: u8, machine: &mut Machine) {
    let base_addr = machine.get_i();
    for i in 0..reg_number + 1{
        let mem_content = machine.read_mem(base_addr + i as u16);
        machine.set_reg(i as usize, mem_content);
    }

}

fn random_instr(instruction: (u16, u16, u16, u16), machine : &mut Machine) {
    let mut rng = rand::thread_rng();
    let nn = instruction.2 << 4 | instruction.3;
    let new_rand = (rng.gen_range(0, 255) & nn) as u8;
    machine.set_reg(instruction.1 as usize, new_rand);
}

fn condition(instruction: (u16, u16, u16, u16), machine : &mut Machine) {
    let reg_x = instruction.1 as usize;
    if instruction.0 == 3 || instruction.0 == 4 {
        let comp_value = (instruction.2 << 4 | instruction.3) as u8;
        let reg_value = machine.get_reg(reg_x);
        if instruction.0 == 3 && comp_value == reg_value {
            machine.incr_pc();
        }
        if instruction.0 == 4 && comp_value != reg_value {
            machine.incr_pc();
        }
        return
    }
    let reg_y = instruction.2 as usize;
    if instruction.0 == 5 && machine.get_reg(reg_x) == machine.get_reg(reg_y) {
            machine.incr_pc();
    }
    if instruction.0 == 8 && machine.get_reg(reg_x) != machine.get_reg(reg_y) {
            machine.incr_pc();
    }
}

fn setter_regs(instruction: (u16, u16, u16, u16), machine : &mut Machine) {
    let reg_x = instruction.1 as usize;
    let regval_x = machine.get_reg(reg_x); 
    let regval_y = machine.get_reg(instruction.2 as usize);
    let new_x_val = match instruction.3 {
                        0 => regval_y, 
                        1 => regval_x | regval_y,
                        2 => regval_x & regval_y,
                        3 => regval_x ^ regval_y,
                        4 => regval_x + regval_y,
                        5 => regval_x - regval_y,
                        6 => regval_x >> 1,
                        7 => regval_y - regval_x ,
                        0xE => regval_x <<  1,
                        _ => {
                            println!("impossible instruction");
                            10
                            },
                    };
    machine.set_reg(reg_x, new_x_val);

}

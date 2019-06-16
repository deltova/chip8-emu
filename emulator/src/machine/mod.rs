mod sdl;

extern crate sdl2; 
extern crate queues;

use std::fmt;
use sdl2::video::Window;
use sdl2::render::Canvas; 
use queues::*;
const SIZE_INSTR: usize = 2;

struct Register {
    data :u8,
}

struct Memory {
    addr_space : Vec<u8>, 
}

pub struct Machine {
    reg_i : u16,
    registers : Vec<Register>,
    memory: Memory,
    stack : Queue<usize>,
    pc : usize,
    canvas : Canvas<Window>,
    sdl_context : sdl2::Sdl,
}

impl Clone for Register {
    #[inline]
    fn clone(&self) ->  Self {
        Register {
            data: self.data,
        }
    }
}

impl Default for Register {
    #[inline]
    fn default() -> Register {
        Register {
            data: 0,
        }
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Default for Memory {
    #[inline]
    fn default() -> Memory {
        Memory {
            addr_space: vec![0; 1000],
        }
    }
}

impl Default for Machine {
    #[inline]
    fn default() -> Machine {
        let (canvas, context) = sdl::init_sdl();
        let machine = Machine {
            registers:  vec![Register::default(); 17],
            stack:  queue![],
            reg_i: 0,
            memory: Memory::default(),
            pc: 512,
            canvas: canvas,
            sdl_context: context,
        };
        machine
    }
}
impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "register i: 0x{:X}\npc: {}\ngeneral registers\n",
               self.reg_i,
               self.pc);
        for (i, reg) in self.registers.iter().enumerate() {
            write!(f, "reg{} = {}\n", i, reg);
        }
        Ok(())
    }
}

impl Machine {
    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn incr_pc(&mut self) {
        self.pc += SIZE_INSTR;
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val as usize;
    }

    pub fn get_reg(&self, idx: usize) -> u8 {
        self.registers[idx].data
    }

    pub fn set_reg(&mut self, idx: usize, val: u8) {
        self.registers[idx].data = val
    }

    pub fn set_i(&mut self, val: u16) {
        self.reg_i = val
    }

    pub fn get_i(&self) -> u16 {
        self.reg_i
    }

    pub fn write_mem(&mut self, idx: u16, val: u8) {
        self.memory.write_mem(idx, val);
    }

    pub fn read_mem(&mut self, idx: u16) -> u8{
        self.memory.read_mem(idx)
    }
    pub fn call(&mut self, addr: u16) {
        self.stack.add(self.pc);
        self.pc = addr as usize;
    }

    pub fn returner(&mut self) {
        let addr_ret = self.stack.remove().expect("cant return without a first call");
        self.pc = addr_ret;
    }

    pub fn clear_screen(&mut self) {
        self.canvas.clear()
    }

    pub fn draw(&mut self, reg1: u8, reg2: u8, n: u8) {
        let x = self.get_reg(reg1 as usize);
        let y = self.get_reg(reg2 as usize);
        let i = self.get_i();
        // for each row of the sprite
        for row in 0..n {
            // get line of pixel from memory
            let mut line = self.read_mem(i + row as u16);
            // go through each pixel of the line
            let mut mask = 1 << 7;
            for col in (0..8) {
                // mask all the bits expect the current one 
                let pixel = line & mask;
                //set pixel
                sdl::draw_pixel(pixel != 0, x + col, y + row, &mut self.canvas);
                // shift the mask to the left
                mask = mask >> 1;
            }
        }
    }
    pub fn dump(&self) {
        print!("{}", self);
    }
}

impl Memory {
    pub fn write_mem(&mut self, idx: u16, val: u8) {
        self.addr_space[idx as usize] = val;
    }
    pub fn read_mem(&mut self, idx: u16) -> u8{
        self.addr_space[idx as usize]
    }
}

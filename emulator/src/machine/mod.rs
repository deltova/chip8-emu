mod sdl;

extern crate sdl2; 
extern crate queues;

use std::fmt;
use sdl2::video::Window;
use sdl2::render::Canvas; 
use queues::*;
use std::vec::Vec;
use std::time::Instant;
use std::time::Duration;
pub const START_ADDR: usize = 512;
const SIZE_INSTR: usize = 2;
// 60Hz
static REFRESH_RATE: u32 = 1000 / 60;

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
    stack : Vec<usize>,
    pc : usize,
    canvas : Canvas<Window>,
    sdl_context : sdl2::Sdl,
    screen_timer : Instant,
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
            stack:  vec![0, 24],
            reg_i: 0,
            memory: Memory::default(),
            pc: START_ADDR,
            canvas: canvas,
            sdl_context: context,
            screen_timer: Instant::now(),
        };
        machine
    }
}
impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "register i: 0x{:X}\npc: {}\ngeneral registers\n",
               self.reg_i,
               self.pc).expect("failed writing on stdout");
        for (i, reg) in self.registers.iter().enumerate() {
            write!(f, "reg{} = {}\n", i, reg).expect("failed writing on stdout");
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
        self.stack.push(self.pc);
        self.pc = addr as usize + START_ADDR;
    }

    pub fn returner(&mut self) {
        let addr_ret = self.stack.pop().expect("cant return without a first call");
        self.pc = START_ADDR;
    }

    pub fn clear_screen(&mut self) {
        self.canvas.clear()
    }

    pub fn draw(&mut self, reg1: u8, reg2: u8, n: u8) {
        let x = self.get_reg(reg1 as usize);
        let y = self.get_reg(reg2 as usize);
        let i = self.get_i();
        let mut rectangles = Vec::new(); 
        // for each row of the sprite
        for row in 0..n {
            // get line of pixel from memory
            let line = self.read_mem(i + row as u16);
            // go through each pixel of the line
            let mut mask = 1 << 7;
            for col in 0..8 {
                // mask all the bits expect the current one 
                let pixel = line & mask;
                //set pixel
                if pixel != 0 {
                    rectangles.push(sdl::draw_pixel(x + col, y + row, &mut self.canvas));
                }
                // shift the mask to the left
                mask = mask >> 1;
            }
        }
        self.canvas.fill_rects(&rectangles).expect("failed rendering rects for texture");
        self.canvas.present();
        let duration = self.screen_timer.elapsed().as_millis() as u32;
        // dont sleep if the rendering time is greater than the refresh rate
        if REFRESH_RATE > duration {
            let time_to_sleep = Duration::from_millis((REFRESH_RATE - duration) as u64);
            std::thread::sleep(time_to_sleep);
        }
        self.screen_timer = Instant::now();
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

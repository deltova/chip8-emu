extern crate queues;
extern crate sdl2;

use crate::emulator::sdl;

use queues::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::fmt;
use std::fs;
use std::time::Duration;
use std::time::Instant;
use std::vec::Vec;

use std::io::BufReader;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{thread, time};

pub const START_ADDR: usize = 512;
const SIZE_INSTR: usize = 2;
// 60Hz
static REFRESH_RATE: u32 = 1000 / 60;

struct Timer {
    pub time: u8,
}

struct Register {
    data: u8,
}

struct Memory {
    addr_space: Vec<u8>,
}

pub struct Machine {
    reg_i: u16,
    registers: Vec<Register>,
    memory: Memory,
    stack: Vec<usize>,
    pc: usize,
    canvas: Canvas<Window>,
    screen_scale: u8,
    sdl_context: sdl2::Sdl,
    screen_timer: Instant,
    threadHandle: Option<thread::JoinHandle<()>>,
    rx: Option<Receiver<u8>>,
    tx: Option<Sender<()>>,
}

impl Clone for Register {
    #[inline]
    fn clone(&self) -> Self {
        Register { data: self.data }
    }
}

impl Default for Register {
    #[inline]
    fn default() -> Register {
        Register { data: 0 }
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
            addr_space: vec![0; 0x1000],
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "register i: 0x{:X}\npc: {}\ngeneral registers\n",
            self.reg_i, self.pc
        )
        .expect("failed writing on stdout");
        for (i, reg) in self.registers.iter().enumerate() {
            write!(f, "reg{} = {}\n", i, reg).expect("failed writing on stdout");
        }
        Ok(())
    }
}

fn run_timer(time: &mut Timer, rx: &Receiver<()>, tx: &Sender<u8>) {
    loop {
        thread::sleep(time::Duration::from_secs(1));
        if time.time != 0 {
            time.time -= 1;
        }
        if !rx.try_recv().is_err() {
            tx.send(time.time).unwrap();
        }
    }
}

impl Machine {
    pub fn new(scale: u8) -> Machine {
        let (canvas, context) = sdl::init_sdl(scale);
        let machine = Machine {
            registers: vec![Register::default(); 17],
            stack: vec![0, 24],
            reg_i: 0,
            memory: Memory::default(),
            pc: START_ADDR,
            canvas: canvas,
            screen_scale: scale,
            sdl_context: context,
            screen_timer: Instant::now(),
            threadHandle: None,
            rx: None,
            tx: None,
        };
        machine
    }
    pub fn set_screen_scale(&mut self, scale: u8) {
        self.screen_scale = scale;
    }

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

    pub fn read_mem(&mut self, idx: u16) -> u8 {
        self.memory.read_mem(idx)
    }
    pub fn call(&mut self, addr: u16) {
        self.stack.push(self.pc);
        self.pc = addr as usize;
    }

    pub fn returner(&mut self) {
        let addr_ret = self.stack.pop().expect("cant return without a first call");
        self.pc = addr_ret;
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
                    rectangles.push(sdl::draw_pixel(
                        x + col,
                        y + row,
                        &mut self.canvas,
                        self.screen_scale,
                    ));
                }
                // shift the mask to the left
                mask = mask >> 1;
            }
        }
        self.canvas
            .fill_rects(&rectangles)
            .expect("failed rendering rects for texture");
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

    pub fn write_rom(&mut self, rom_path: &str) {
        let instructions = fs::read(&rom_path).expect("Unable to read file");
        for (i, instr) in instructions.iter().enumerate() {
            self.write_mem((START_ADDR + i) as u16, *instr);
        }
    }

    pub fn init_timer(&mut self, reg: Option<u8>) {
        let (time_sender, time_receiver) = mpsc::channel();
        let (notification_sender, notification_receiver) = mpsc::channel();
        let mut time = Timer { time: 10 };
        if let Some(register) = reg {
            time.time = self.get_reg(register as usize);
        }
        self.threadHandle = Some(thread::spawn(move || {
            run_timer(&mut time, &notification_receiver, &time_sender);
        }));
        self.tx = Some(notification_sender);
        self.rx = Some(time_receiver);
    }

    pub fn get_time(&mut self, reg: u8) {
        if let Some(sender) = self.tx.take() {
            sender.send(()).unwrap();
        }
        if let Some(receiver) = self.rx.take() {
            let val = receiver.recv().unwrap();
            self.set_reg(reg as usize, val);
        }
        else {
            self.set_reg(reg as usize, 0);
        }
    }

    pub fn join_timer(&mut self) {
        if let Some(handle) = self.threadHandle.take() {
            handle.join().unwrap();
        }
    }
}

impl Memory {
    pub fn write_mem(&mut self, idx: u16, val: u8) {
        self.addr_space[idx as usize] = val;
    }
    pub fn read_mem(&mut self, idx: u16) -> u8 {
        self.addr_space[idx as usize]
    }
}

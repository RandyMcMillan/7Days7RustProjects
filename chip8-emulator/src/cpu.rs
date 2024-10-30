use minifb::Key;

use crate::audio::Audio;
use crate::util::{get_bit, get_hex_digits};
use crate::window::Window;

const RAM_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;
const RUNLOOP_TIMER_DEFAULT: usize = 8;
const PROGRAM_START: usize = 0x200;

const RAM_DIGITS: [[u8; 5]; 16] = [
    [0xf0, 0x90, 0x90, 0x90, 0xf0],
    [0x20, 0x60, 0x20, 0x20, 0x70],
    [0xf0, 0x10, 0xf0, 0x80, 0xf0],
    [0xf0, 0x10, 0xf0, 0x10, 0xf0],
    [0x90, 0x90, 0xf0, 0x10, 0x10],
    [0xf0, 0x80, 0xf0, 0x10, 0xf0],
    [0xf0, 0x80, 0xf0, 0x90, 0xf0],
    [0xf0, 0x10, 0x20, 0x40, 0x40],
    [0xf0, 0x90, 0xf0, 0x90, 0xf0],
    [0xf0, 0x90, 0xf0, 0x10, 0xf0],
    [0xf0, 0x90, 0xf0, 0x90, 0x90],
    [0xe0, 0x90, 0xe0, 0x90, 0xe0],
    [0xf0, 0x80, 0x80, 0x80, 0xf0],
    [0xe0, 0x90, 0x90, 0x90, 0xe0],
    [0xf0, 0x80, 0xf0, 0x80, 0xf0],
    [0xf0, 0x80, 0xf0, 0x80, 0x80],
];

pub struct CPU {
    ram: [u8; RAM_SIZE],
    v: [u8; REGISTER_COUNT],
    i: usize,
    dt: u8,
    st: u8,
    stack: [usize; STACK_SIZE],
    sp: usize,
    pc: usize,
    win: Window,
    audio: Audio,
}

impl CPU {
    pub fn new(win: Window, audio: Audio) -> CPU {
        let mut ret = CPU {
            ram: [0; RAM_SIZE],
            v: [0; REGISTER_COUNT],
            i: 0,
            dt: 0,
            st: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            pc: PROGRAM_START,
            win,
            audio,
        };
        ret.preload_ram();
        ret
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) -> Result<(), &str> {
        if PROGRAM_START + rom.len() >= RAM_SIZE {
            return Err("Out of memory: program too large");
        }
        for (j, c) in rom.into_iter().enumerate() {
            self.ram[j + PROGRAM_START] = *c;
        }
        Ok(())
    }

    fn preload_ram(&mut self) {
        for (j, d) in RAM_DIGITS.iter().enumerate() {
            for (k, b) in d.iter().enumerate() {
                self.ram[(0x10 * j) + k] = *b;
            }
        }
    }

    pub fn run_loop(&mut self) -> Result<(), &str> {
        let mut executing = true;
        let mut waiting_for_keypress = false;
        let mut store_keypress_in: usize = 0x0;
        let mut time_to_runloop: usize = RUNLOOP_TIMER_DEFAULT;

        while self.win.is_open() && !self.win.is_key_down(Key::Escape) && self.pc <= RAM_SIZE {
            let keys_pressed = self.win.handle_key_events();

            for (j, k) in keys_pressed.iter().enumerate() {
                if *k {
                    if waiting_for_keypress {
                        executing = true;
                        waiting_for_keypress = false;
                        self.v[store_keypress_in] = j as u8;
                        break;
                    }
                    println!("{:01x} pressed!", j);
                }
            }

            let b1 = self.ram[self.pc] as u16;
            let b2 = self.ram[self.pc + 1] as u16;
            let instruction = (b1 * 256) + b2;

            let mut next_instruction = true;

            if executing {
                println!(
                    "{:03x}, {:04x}, {:04x}, {:02x?}",
                    self.pc, instruction, self.i, self.v
                );
                match instruction {
                    0x00e0 => {
                        self.win.clear_screen();
                    }
                    0x00ee => {
                        if self.sp == 0 {
                            return Err("Stack empty, cannot return from subroutine!");
                        }
                        self.sp -= 1;
                        self.pc = self.stack[self.sp];
                    }
                    0x1000..=0x1fff => {
                        self.pc = get_hex_digits(&instruction, 3, 0);
                        next_instruction = false;
                    }
                    0x2000..=0x2fff => {
                        let loc = get_hex_digits(&instruction, 3, 0);
                        if self.sp == STACK_SIZE {
                            return Err("Stack full, cannot push!");
                        }
                        self.stack[self.sp] = self.pc;
                        self.sp += 1;
                        self.pc = loc;
                        next_instruction = false;
                    }
                    0x3000..=0x3fff => {
                        let val = get_hex_digits(&instruction, 2, 0);
                        let reg = get_hex_digits(&instruction, 1, 2);
                        if self.v[reg] == val as u8 {
                            self.pc += 2;
                        }
                    }
                    0x4000..=0x4fff => {
                        let val = get_hex_digits(&instruction, 2, 0);
                        let reg = get_hex_digits(&instruction, 1, 2);
                        if self.v[reg] != val as u8 {
                            self.pc += 2;
                        }
                    }
                    0x5000..=0x5fff => {
                        let reg1 = get_hex_digits(&instruction, 1, 2);
                        let reg2 = get_hex_digits(&instruction, 1, 1);
                        if self.v[reg1] == self.v[reg2] {
                            self.pc += 2;
                        }
                    }
                    0x6000..=0x6fff => {
                        let val = get_hex_digits(&instruction, 2, 0);
                        let reg = get_hex_digits(&instruction, 1, 2);
                        self.v[reg] = val as u8;
                    }
                    0x7000..=0x7fff => {
                        let val = get_hex_digits(&instruction, 2, 0);
                        let reg = get_hex_digits(&instruction, 1, 2);
                        self.v[reg] = self.v[reg].overflowing_add(val as u8).0;
                    }
                    0x8000..=0x8fff => {
                        let lsb = get_hex_digits(&instruction, 1, 0);
                        let reg1 = get_hex_digits(&instruction, 1, 2);
                        let reg2 = get_hex_digits(&instruction, 1, 1);

                        match lsb {
                            0x0 => {
                                self.v[reg1] = self.v[reg2];
                            }
                            0x1 => {
                                self.v[reg1] |= self.v[reg2];
                            }
                            0x2 => {
                                self.v[reg1] &= self.v[reg2];
                            }
                            0x3 => {
                                self.v[reg1] ^= self.v[reg2];
                            }
                            0x4 => {
                                let (res, over) = self.v[reg1].overflowing_add(self.v[reg2]);
                                self.v[reg1] = res;
                                self.v[0xf] = if over { 1 } else { 0 };
                            }
                            0x5 => {
                                let (res, over) = self.v[reg1].overflowing_sub(self.v[reg2]);
                                self.v[reg1] = res;
                                self.v[0xf] = if over { 0 } else { 1 };
                            }
                            0x6 => {
                                let res = self.v[reg1].overflowing_shr(1).0;
                                self.v[0xf] = get_bit(&self.v[reg1], 0);
                                self.v[reg1] = res;
                            }
                            0x7 => {
                                let (res, over) = self.v[reg2].overflowing_sub(self.v[reg1]);
                                self.v[reg1] = res;
                                self.v[0xf] = if over { 0 } else { 1 };
                            }
                            0xe => {
                                let res = self.v[reg1].overflowing_shl(1).0;
                                self.v[0xf] = get_bit(&self.v[reg1], 7);
                                self.v[reg1] = res;
                            }
                            _ => {
                                println!("Warning: unrecognized instruction: {:04x}", instruction);
                            }
                        };
                    }
                    0x9000..=0x9fff => {
                        let reg1 = get_hex_digits(&instruction, 1, 2);
                        let reg2 = get_hex_digits(&instruction, 1, 1);
                        if self.v[reg1] != self.v[reg2] {
                            self.pc += 2;
                        }
                    }
                    0xa000..=0xafff => {
                        self.i = get_hex_digits(&instruction, 3, 0);
                    }
                    0xb000..=0xbfff => {
                        self.pc = get_hex_digits(&instruction, 3, 0) + self.v[0] as usize;
                        next_instruction = false;
                    }
                    0xc000..=0xcfff => {
                        let rnd = rand::random::<u8>();
                        let val = get_hex_digits(&instruction, 2, 0);
                        let reg = get_hex_digits(&instruction, 1, 2);
                        self.v[reg] = rnd & val as u8;
                    }
                    0xd000..=0xdfff => {
                        let reg1 = get_hex_digits(&instruction, 1, 2);
                        let reg2 = get_hex_digits(&instruction, 1, 1);
                        let init_x = self.v[reg1];
                        let init_y = self.v[reg2];
                        let mut byte_count = get_hex_digits(&instruction, 1, 0);
                        let mut bytes_to_print: Vec<u8> = Vec::new();
                        let mut j = 0;
                        while byte_count > 0 {
                            bytes_to_print.push(self.ram[self.i + j]);
                            byte_count -= 1;
                            j += 1;
                        }
                        self.v[0xf] = self.win.draw(&bytes_to_print, init_x, init_y);
                    }
                    0xe000..=0xff65 => {
                        let d1 = get_hex_digits(&instruction, 1, 3);
                        let d2 = get_hex_digits(&instruction, 1, 2);
                        let d3 = get_hex_digits(&instruction, 1, 1);
                        let d4 = get_hex_digits(&instruction, 1, 0);

                        if d1 == 0xe && d3 == 0x9 && d4 == 0xe {
                            if keys_pressed[self.v[d2] as usize] {
                                self.pc += 2;
                            }
                        } else if d1 == 0xe && d3 == 0xa && d4 == 0x1 {
                            if !keys_pressed[self.v[d2] as usize] {
                                self.pc += 2;
                            }
                        } else if d1 == 0xf && d3 == 0x0 && d4 == 0x7 {
                            self.v[d2] = self.dt;
                        } else if d1 == 0xf && d3 == 0x0 && d4 == 0xa {
                            executing = false;
                            waiting_for_keypress = true;
                            store_keypress_in = d2;
                        } else if d1 == 0xf && d3 == 0x1 && d4 == 0x5 {
                            self.dt = self.v[d2];
                        } else if d1 == 0xf && d3 == 0x1 && d4 == 0x8 {
                            self.st = self.v[d2];
                        } else if d1 == 0xf && d3 == 0x1 && d4 == 0xe {
                            self.i += self.v[d2] as usize;
                        } else if d1 == 0xf && d3 == 0x2 && d4 == 0x9 {
                            self.i = (0x10 * self.v[d2]) as usize;
                        } else if d1 == 0xf && d3 == 0x3 && d4 == 0x3 {
                            self.ram[self.i] = self.v[d2] / 100;
                            self.ram[self.i + 1] = (self.v[d2] % 100) / 10;
                            self.ram[self.i + 2] = self.v[d2] % 10;
                        } else if d1 == 0xf && d3 == 0x5 && d4 == 0x5 {
                            for j in 0..=d2 {
                                self.ram[self.i + j] = self.v[j];
                            }
                        } else if d1 == 0xf && d3 == 0x6 && d4 == 0x5 {
                            for j in 0..=d2 {
                                self.v[j] = self.ram[self.i + j];
                            }
                        } else {
                            println!("Warning: unrecognized instruction: {:04x}", instruction);
                        }
                    }
                    _ => {
                        println!("Warning: unrecognized instruction: {:04x}", instruction);
                    }
                };

                if next_instruction {
                    self.pc += 2;
                }
            }

            if time_to_runloop == 0 {
                if self.dt > 0 {
                    self.dt -= 1;
                }

                if self.st > 0 {
                    self.audio.play();
                    self.st -= 1;
                } else if self.st == 0 {
                    self.audio.pause();
                }

                self.win.refresh();

                time_to_runloop = RUNLOOP_TIMER_DEFAULT;
            } else {
                time_to_runloop -= 1;
            }
        }
        Ok(())
    }
}

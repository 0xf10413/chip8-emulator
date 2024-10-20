const CHIP8_MEMORY_SIZE: usize = 4096;
const CHIP8_FIRST_BYTE_ADDRESS: usize = 512;
const CHIP8_NUMBER_REGISTERS: usize = 16;
pub const CHIP8_SCREEN_WIDTH: usize = 64;
pub const CHIP8_SCREEN_HEIGHT: usize = 32;
const CHIP8_CALL_STACK_MAX_DEPTH: usize = 16;
const CHIP8_NUMBER_KEYS: usize = 16;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum OpCode {
    OC_0NNN(u16),
    OC_00E0,
    OC_00EE,
    OC_1NNN(usize),
    OC_2NNN(usize),
    OC_3XNN(usize, u8),
    OC_4XNN(usize, u8),
    OC_5XY0(usize, usize),
    OC_6XNN(usize, u8),
    OC_7XNN(usize, u8),
    OC_8XY0(usize, usize),
    OC_8XY1(usize, usize),
    OC_8XY2(usize, usize),
    OC_8XY3(usize, usize),
    OC_8XY4(usize, usize),
    OC_8XY5(usize, usize),
    OC_8XY6(usize, usize),
    OC_8XY7(usize, usize),
    OC_8XYE(usize, usize),
    OC_9XY0(usize, usize),
    OC_ANNN(usize),
    // BNNN
    OC_CXNN(usize, u8),
    OC_DXYN(usize, usize, usize),
    OC_EX9E(usize),
    OC_EXA1(usize),
    OC_FX07(usize),
    // FX0A
    OC_FX15(usize),
    OC_FX18(usize),
    OC_FX1E(usize),
    // FX29
    // FX33
    OC_FX55(usize),
    OC_FX65(usize),
}

fn parse_opcode(raw_opcode: u16) -> Option<OpCode> {
    // 00E0
    if raw_opcode == 0x00E0 {
        return Some(OpCode::OC_00E0);
    }

    // 00EE
    if raw_opcode == 0x00EE {
        return Some(OpCode::OC_00EE);
    }

    // 0NNN
    if raw_opcode & 0xF000 == 0x0000 {
        let nnn: u16 = raw_opcode & 0x0FFF;
        return Some(OpCode::OC_0NNN(nnn));
    }

    // 1NNN
    if raw_opcode & 0xF000 == 0x1000 {
        let nnn: usize = (0x0FFF & raw_opcode) as usize;
        return Some(OpCode::OC_1NNN(nnn));
    }

    // 2NNN
    if raw_opcode & 0xF000 == 0x2000 {
        let nnn: usize = (0x0FFF & raw_opcode) as usize;
        return Some(OpCode::OC_2NNN(nnn));
    }

    // 3XNN
    if raw_opcode & 0xF000 == 0x3000 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let nn: u8 = (0x00FF & raw_opcode) as u8;
        return Some(OpCode::OC_3XNN(x, nn));
    }

    // 4XNN
    if raw_opcode & 0xF000 == 0x4000 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let nn: u8 = (0x00FF & raw_opcode) as u8;
        return Some(OpCode::OC_4XNN(x, nn));
    }

    // 5XY0
    if raw_opcode & 0xF00F == 0x5000 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_5XY0(x, y));
    }

    // 6XNN
    if raw_opcode & 0xF000 == 0x6000 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let nn: u8 = (0x00FF & raw_opcode) as u8;
        return Some(OpCode::OC_6XNN(x, nn));
    }

    // 7XNN
    if raw_opcode & 0xF000 == 0x7000 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let nn: u8 = (0x00FF & raw_opcode) as u8;
        return Some(OpCode::OC_7XNN(x, nn));
    }

    // 8XY0
    if raw_opcode & 0xF00F == 0x8000 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_8XY0(x, y));
    }

    // 8XY1
    if raw_opcode & 0xF00F == 0x8001 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_8XY1(x, y));
    }

    // 8XY2
    if raw_opcode & 0xF00F == 0x8002 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_8XY2(x, y));
    }

    // 8XY3
    if raw_opcode & 0xF00F == 0x8003 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_8XY3(x, y));
    }

    // 8XY4
    if raw_opcode & 0xF00F == 0x8004 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_8XY4(x, y));
    }

    // 8XY5
    if raw_opcode & 0xF00F == 0x8005 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_8XY5(x, y));
    }

    // 8XY6
    if raw_opcode & 0xF00F == 0x8006 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_8XY6(x, y));
    }

    // 8XY7
    if raw_opcode & 0xF00F == 0x8007 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_8XY7(x, y));
    }

    // 8XYE
    if raw_opcode & 0xF00F == 0x800E {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_8XYE(x, y));
    }

    // 9XY0
    if raw_opcode & 0xF00F == 0x9000 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        return Some(OpCode::OC_9XY0(x, y));
    }

    // ANNN
    if raw_opcode & 0xF000 == 0xA000 {
        let nnn: usize = (0x0FFF & raw_opcode) as usize;
        return Some(OpCode::OC_ANNN(nnn));
    }

    // CXNN
    if raw_opcode & 0xF000 == 0xC000 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let nn: u8 = (0x00FF & raw_opcode) as u8;
        return Some(OpCode::OC_CXNN(x, nn));
    }

    // DXYN
    if raw_opcode & 0xF000 == 0xD000 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let y: usize = ((0x00F0 & raw_opcode) >> 4) as usize;
        let n: usize = ((0x000F & raw_opcode) >> 0) as usize;
        return Some(OpCode::OC_DXYN(x, y, n));
    }

    // EX9E
    if raw_opcode & 0xF0FF == 0xE09E {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        return Some(OpCode::OC_EX9E(x));
    }

    // EXA1
    if raw_opcode & 0xF0FF == 0xE0A1 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        return Some(OpCode::OC_EXA1(x));
    }

    // FX07
    if raw_opcode & 0xF0FF == 0xF007 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        return Some(OpCode::OC_FX07(x));
    }

    // FX15
    if raw_opcode & 0xF0FF == 0xF015 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        return Some(OpCode::OC_FX15(x));
    }

    // FX18
    if raw_opcode & 0xF0FF == 0xF018 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        return Some(OpCode::OC_FX18(x));
    }

    // FX1E
    if raw_opcode & 0xF0FF == 0xF01E {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        return Some(OpCode::OC_FX1E(x));
    }

    // FX55
    if raw_opcode & 0xF0FF == 0xF055 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        return Some(OpCode::OC_FX55(x));
    }

    // FX65
    if raw_opcode & 0xF0FF == 0xF065 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        return Some(OpCode::OC_FX65(x));
    }

    return None;
}

#[derive(PartialEq, Debug)]
pub enum PixelStatus {
    Black,
    White,
}

pub struct Emulator {
    memory: [u8; CHIP8_MEMORY_SIZE],
    program_counter: usize,
    generic_registers: [u8; CHIP8_NUMBER_REGISTERS],
    memory_register: usize,
    pub screen: [PixelStatus; (CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT) as usize],
    call_stack: [usize; CHIP8_CALL_STACK_MAX_DEPTH],
    call_stack_depth: usize,
    pub keys_pressed: [bool; CHIP8_NUMBER_KEYS],
    pub system_clock: u8,
    pub sound_clock: u8,
}

const SCREEN_ARRAY_REPEAT_VALUE: PixelStatus = PixelStatus::Black;
impl Emulator {
    pub fn new() -> Self {
        Self {
            memory: [0; CHIP8_MEMORY_SIZE],
            program_counter: CHIP8_FIRST_BYTE_ADDRESS,
            generic_registers: [0; CHIP8_NUMBER_REGISTERS],
            memory_register: 0,
            screen: [SCREEN_ARRAY_REPEAT_VALUE;
                (CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT) as usize],
            call_stack: [0; CHIP8_CALL_STACK_MAX_DEPTH],
            call_stack_depth: 0,
            keys_pressed: [false; CHIP8_NUMBER_KEYS],
            system_clock: 0,
            sound_clock: 0,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.memory[CHIP8_FIRST_BYTE_ADDRESS + i] = *byte
        }
        self.program_counter = CHIP8_FIRST_BYTE_ADDRESS;
    }

    pub fn process_next_instruction(&mut self) {
        println!("Reading code and processing next instruction...");

        // Read next, which is build from the next two bytes
        let opcode_first_part: u16 = self.memory[self.program_counter] as u16;
        let opcode_second_part: u16 = self.memory[self.program_counter + 1] as u16;
        let opcode_raw: u16 = (opcode_first_part << 8) + opcode_second_part;
        println!("Bytes read, to be parsed as opcode: {:#06X}", opcode_raw);

        // Parse what we just read
        let identified_opcode = parse_opcode(opcode_raw);

        // Process the new opcode
        match identified_opcode {
            Some(ref opcode) => {
                println!("Identified read opcode as {:?}", opcode);
                self.process_opcode(opcode);
            }
            None => panic!("Unidentified opcode 0x{:X}!", opcode_raw),
        }

        // Jump to next instruction
        self.program_counter += 2;
    }

    fn process_opcode(&mut self, opcode: &OpCode) {
        match opcode {
            OpCode::OC_00E0 => {
                // Clears screen
                println!("Clearing screen");
                self.screen = [SCREEN_ARRAY_REPEAT_VALUE;
                    (CHIP8_SCREEN_WIDTH * CHIP8_SCREEN_HEIGHT) as usize];
            }

            OpCode::OC_00EE => {
                // Jumps back in the call stack
                println!("Jumping back in call stack");
                self.call_stack_depth -= 1;
                self.program_counter = self.call_stack[self.call_stack_depth];
            }

            OpCode::OC_0NNN(_) => {
                // Calls code routine at address NNN
                panic!("OpCode 0NNN not implemented!")
            }

            OpCode::OC_1NNN(nnn) => {
                // Next instruction will be at address NNN
                println!("Setting pc to {}", nnn);
                self.program_counter = *nnn - 2; // TODO: increase pc in this function to avoid hack?
            }

            OpCode::OC_2NNN(nnn) => {
                // Next instruction will be at address NNN.
                // However, this time, we keep the previous pc value.
                println!("Jumping to {} while increasing call stack", nnn);
                self.call_stack[self.call_stack_depth] = self.program_counter;
                self.call_stack_depth += 1;
                self.program_counter = *nnn - 2; // TODO: increase pc in this function to avoid hack?
            }

            OpCode::OC_3XNN(x, nn) => {
                // Next instruction will be skipped if VX == NN
                println!("Skipping next instruction if V{:X} == {}", x, nn);
                if self.generic_registers[*x] == *nn {
                    self.program_counter += 2;
                }
            }

            OpCode::OC_4XNN(x, nn) => {
                // Next instruction will be skipped if VX != NN
                println!("Skipping next instruction if V{:X} != {}", x, nn);
                if self.generic_registers[*x] != *nn {
                    self.program_counter += 2;
                }
            }

            OpCode::OC_5XY0(x, y) => {
                // Skips next instruction if VX == VY
                println!("Skipping next instruction if V{:X} == V{:X}", x, y);
                if self.generic_registers[*x] == self.generic_registers[*y] {
                    self.program_counter += 2;
                }
            }

            OpCode::OC_6XNN(x, nn) => {
                // Defines register VX to NN
                println!("Setting register V{:X} to {:#X}", x, nn);
                self.generic_registers[*x] = *nn;
            }

            OpCode::OC_7XNN(x, nn) => {
                // Adds NN to register VX
                println!("Adding {:#X} to register V{:X}", nn, x);
                self.generic_registers[*x] = self.generic_registers[*x].wrapping_add(*nn);
            }

            OpCode::OC_8XY0(x, y) => {
                // Set register VX to the value of register VY
                println!("Setting register V{:X} to the value of V{:X}", x, y);
                self.generic_registers[*x] = self.generic_registers[*y];
            }

            OpCode::OC_8XY1(x, y) => {
                // Set register VX to the value of VX | VY
                println!("OR'ing register V{:X} with the value of V{:X}", x, y);
                self.generic_registers[*x] |= self.generic_registers[*y];
            }

            OpCode::OC_8XY2(x, y) => {
                // Set register VX to the value of VX & VY
                println!("AND'ing register V{:X} with the value of V{:X}", x, y);
                self.generic_registers[*x] &= self.generic_registers[*y];
            }

            OpCode::OC_8XY3(x, y) => {
                // Set register VX to the value of VX ^ VY
                println!("XOR'ing register V{:X} with the value of V{:X}", x, y);
                self.generic_registers[*x] ^= self.generic_registers[*y];
            }

            OpCode::OC_8XY4(x, y) => {
                // Set register VX to the value of VX + VY, write carry in VF
                println!(
                    "Adding register V{:X} with the value of V{:X}, while putting carry in VF",
                    x, y
                );
                let (result, carry) =
                    self.generic_registers[*x].overflowing_add(self.generic_registers[*y]);
                self.generic_registers[*x] = result;
                self.generic_registers[0xF] = if carry { 1 } else { 0 };
            }

            OpCode::OC_8XY5(x, y) => {
                // Set register VX to the value of VX - VY, write carry in VF
                println!("Substracting register V{:X} with the value of V{:X}, while putting carry in VF", x, y);
                let (result, carry) =
                    self.generic_registers[*x].overflowing_sub(self.generic_registers[*y]);
                self.generic_registers[*x] = result;
                self.generic_registers[0xF] = if carry { 0 } else { 1 };
            }

            OpCode::OC_8XY6(x, ..) => {
                // Shifts VX to the right by 1 bit. VF will contain the lost bit.
                // Somehow, Y is not used?
                println!(
                    "Shifting right register V{:X} with the lost bit written in VF",
                    x
                );
                self.generic_registers[0xF] = self.generic_registers[*x] & 0x01;
                self.generic_registers[*x] >>= 1;
            }

            OpCode::OC_8XY7(x, y) => {
                // Sets VX to VY - VX. VF is set to 0 if there is an overflow, 1 otherwise.
                println!("Setting register V{:X} to V{:X} - V{:X} with opposite of overflow written in VF", x, y, x);
                let (result, overflow) =
                    self.generic_registers[*y].overflowing_sub(self.generic_registers[*x]);
                self.generic_registers[*x] = result;
                self.generic_registers[0xF] = if overflow { 0 } else { 1 };
            }

            OpCode::OC_8XYE(x, ..) => {
                // Shifts VX to the left by 1 bit. VF will contain the lost bit.
                // Somehow, Y is not used?
                println!(
                    "Shifting left register V{:X} with the lost bit written in VF",
                    x
                );
                self.generic_registers[0xF] = (self.generic_registers[*x] & 0b10000000) >> 7;
                self.generic_registers[*x] <<= 1;
            }

            OpCode::OC_9XY0(x, y) => {
                // Skips next instruction if VX != VY
                println!("Skipping next instruction if V{:X} != V{:X}", x, y);
                if self.generic_registers[*x] != self.generic_registers[*y] {
                    self.program_counter += 2;
                }
            }

            OpCode::OC_ANNN(nnn) => {
                // Set register I to NNN
                println!("Setting I to {}", nnn);
                self.memory_register = *nnn;
            }

            OpCode::OC_CXNN(x, nn) => {
                // Set register VX to a random number between 0 and nn
                println!("Setting V{:x} to a random number less than {}", x, nn);
                self.generic_registers[*x] = rand::random::<u8>() & nn;
            }

            OpCode::OC_DXYN(x, y, n) => {
                // Draw sprite with height n at coordinates (VX, VY)
                println!(
                    "Drawing sprite with height {} at (V{:x} = {}, V{:x} = {})",
                    n, x, self.generic_registers[*x], y, self.generic_registers[*y]
                );
                let pos_x = self.generic_registers[*x] as usize;
                let pos_y = self.generic_registers[*y] as usize;
                let mut any_pixel_turned_off = false;
                for (offset_y, byte) in self.memory[self.memory_register..self.memory_register + n]
                    .iter()
                    .enumerate()
                {
                    for bit_index in (0..8).rev() {
                        let offset_x = 7 - bit_index;
                        let switch_pixel = (byte & (1 << bit_index)) >> bit_index == 1;
                        if switch_pixel {
                            let pixel_coordinate: usize = (pos_y + offset_y)
                                * (CHIP8_SCREEN_WIDTH as usize)
                                + pos_x
                                + offset_x;
                            match self.screen[pixel_coordinate] {
                                PixelStatus::Black => {
                                    self.screen[pixel_coordinate] = PixelStatus::White
                                }
                                PixelStatus::White => {
                                    self.screen[pixel_coordinate] = PixelStatus::Black;
                                    any_pixel_turned_off = true;
                                }
                            }
                        }
                    }
                }
                self.generic_registers[0xF] = if any_pixel_turned_off { 1 } else { 0 };
            }

            OpCode::OC_EX9E(x) => {
                // Skips next instruction if key indicated by VX is pressed
                println!("Skipping next instruction if V{:X}'s key is pressed", x);
                if self.keys_pressed[self.generic_registers[*x] as usize] {
                    self.program_counter += 2;
                }
            }

            OpCode::OC_EXA1(x) => {
                // Skips next instruction if key indicated by VX is *not* pressed
                println!("Skipping next instruction if V{:X}'s key is pressed", x);
                if !self.keys_pressed[self.generic_registers[*x] as usize] {
                    self.program_counter += 2;
                }
            }

            OpCode::OC_FX07(x) => {
                // Sets VX to the current value of the system clock
                println!("Setting V{:X} to the current value of system clock", x);
                self.generic_registers[*x] = self.system_clock;
            }

            OpCode::OC_FX18(x) => {
                // Sets the sound clock to the current value of VX
                println!("Setting the sound clock to the current value of V{:X}", x);
                self.sound_clock = self.generic_registers[*x];
            }

            OpCode::OC_FX15(x) => {
                // Sets the system clock to the current value of VX
                println!("Setting the system clock to the current value of V{:X}", x);
                self.system_clock = self.generic_registers[*x];
            }

            OpCode::OC_FX1E(x) => {
                // Add VX to I, taking into account overflow. Writes overflow in VF.
                println!(
                    "Adding V{:X} to I, writing overflow in VF in memory at I",
                    x
                );
                self.memory_register += self.generic_registers[*x] as usize;
                if self.memory_register > 0xFFF {
                    self.memory_register -= 0xFFF;
                    self.generic_registers[0xF] = 1;
                } else {
                    self.generic_registers[0xF] = 0;
                }
            }

            OpCode::OC_FX55(x) => {
                // Load bytes in V0, ..., VX in memory at I
                println!("Loading V0, ..., V{:X} in memory at I", x);
                for i in 0..=*x {
                    println!(
                        "Loading V{:x} {:b} at {:x}",
                        i,
                        self.generic_registers[i],
                        self.memory_register + i
                    );
                    self.memory[self.memory_register + i] = self.generic_registers[i];
                }
            }

            OpCode::OC_FX65(x) => {
                // Load bytes in memory at I into V0, ..., VX
                println!("Loading bytes from I into V0, ..., V{:X}", x);
                for i in 0..=*x {
                    self.generic_registers[i] = self.memory[self.memory_register + i];
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Unidentified opcode 0x800F!")]
    fn test_unknown_opcode() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x80, 0x0F]);
        emulator.process_next_instruction();
    }

    #[test]
    #[allow(non_snake_case)]
    #[should_panic(expected = "OpCode 0NNN not implemented!")]
    fn test_opcode_0NNN() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x00, 0x00]);
        emulator.process_next_instruction();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_00E0() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x00, 0xE0]);
        emulator.screen[0x10] = PixelStatus::White;
        emulator.process_next_instruction();
        assert_eq!(emulator.screen[0x10], PixelStatus::Black);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_1NNN() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x12, 0xFF]);
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, 0x2FF);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_2NNN_00EE() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x22, 0x04, 0x00, 0x00, 0x00, 0xEE]);
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, 0x204);
        assert_eq!(emulator.call_stack[0], CHIP8_FIRST_BYTE_ADDRESS);
        assert_eq!(emulator.call_stack_depth, 1);
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, 0x202);
        assert_eq!(emulator.call_stack_depth, 0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_3XNN() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0x01, 0x3A, 0x01, 0x6A, 0x10, 0x3A, 0x02]);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2);
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
        assert_eq!(emulator.generic_registers[0xA], 0x01);
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_4XNN() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0x01, 0x4A, 0x00, 0x6A, 0x10, 0x4A, 0x01]);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2);
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
        assert_eq!(emulator.generic_registers[0xA], 0x01);
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_5XY0() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[
            0x6A, 0x01, 0x6B, 0x01, 0x5A, 0xB0, 0x6A, 0x10, 0x6A, 0x02, 0x5A, 0xB0,
        ]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x01);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
        assert_eq!(emulator.generic_registers[0xA], 0x01);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x02);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 10);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x02);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 12);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_6XNN() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0x15]);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x15);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_7XNN() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x7B, 0x03, 0x7B, 0x05]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xB], 0x08);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XY0() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0x03, 0x6B, 0x05, 0x8A, 0xB0]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x03);
        assert_eq!(emulator.generic_registers[0xB], 0x05);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x05);
        assert_eq!(emulator.generic_registers[0xB], 0x05);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XY1() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0x03, 0x6B, 0x30, 0x8A, 0xB1]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x03);
        assert_eq!(emulator.generic_registers[0xB], 0x30);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x33);
        assert_eq!(emulator.generic_registers[0xB], 0x30);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XY2() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0b0011, 0x6B, 0b0101, 0x8A, 0xB2]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0b0011);
        assert_eq!(emulator.generic_registers[0xB], 0b0101);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0b0001);
        assert_eq!(emulator.generic_registers[0xB], 0b0101);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XY3() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0b0011, 0x6B, 0b0101, 0x8A, 0xB3]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0b0011);
        assert_eq!(emulator.generic_registers[0xB], 0b0101);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0b0110);
        assert_eq!(emulator.generic_registers[0xB], 0b0101);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XY4() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0xFE, 0x6B, 0x01, 0x6F, 0x10, 0x8A, 0xB4, 0x8A, 0xB4]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0xFE);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        assert_eq!(emulator.generic_registers[0xF], 0x10);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0xFF);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        assert_eq!(emulator.generic_registers[0xF], 0x00);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x00);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        assert_eq!(emulator.generic_registers[0xF], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 10);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XY5() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0x03, 0x6B, 0x02, 0x6F, 0x10, 0x8A, 0xB5, 0x8A, 0xB5]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x03);
        assert_eq!(emulator.generic_registers[0xB], 0x02);
        assert_eq!(emulator.generic_registers[0xF], 0x10);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x01);
        assert_eq!(emulator.generic_registers[0xB], 0x02);
        assert_eq!(emulator.generic_registers[0xF], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0xFF);
        assert_eq!(emulator.generic_registers[0xB], 0x02);
        assert_eq!(emulator.generic_registers[0xF], 0x00);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 10);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XY6() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0b0110, 0x6F, 0x10, 0x8A, 0xB6, 0x8A, 0xB6]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0b0110);
        assert_eq!(emulator.generic_registers[0xF], 0x10);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0b0011);
        assert_eq!(emulator.generic_registers[0xF], 0x00);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0b0001);
        assert_eq!(emulator.generic_registers[0xF], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XY7() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[
            0x6A, 0x04, 0x6B, 0x03, 0x6F, 0x10, 0x8A, 0xB7, 0x6A, 0x01, 0x8A, 0xB7,
        ]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x04);
        assert_eq!(emulator.generic_registers[0xB], 0x03);
        assert_eq!(emulator.generic_registers[0xF], 0x10);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0xFF);
        assert_eq!(emulator.generic_registers[0xB], 0x03);
        assert_eq!(emulator.generic_registers[0xF], 0x00);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x01);
        assert_eq!(emulator.generic_registers[0xB], 0x03);
        assert_eq!(emulator.generic_registers[0xF], 0x00);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 10);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x02);
        assert_eq!(emulator.generic_registers[0xB], 0x03);
        assert_eq!(emulator.generic_registers[0xF], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 12);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XYE() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x6A, 0b10100000, 0x6F, 0x10, 0x8A, 0xBE, 0x8A, 0xBE]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0b10100000);
        assert_eq!(emulator.generic_registers[0xF], 0x10);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0b01000000);
        assert_eq!(emulator.generic_registers[0xF], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0b10000000);
        assert_eq!(emulator.generic_registers[0xF], 0x00);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_9XY0() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[
            0x6A, 0x02, 0x6B, 0x01, 0x9A, 0xB0, 0x6A, 0x10, 0x6A, 0x01, 0x9A, 0xB0,
        ]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x02);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
        assert_eq!(emulator.generic_registers[0xA], 0x02);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x01);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 10);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x01);
        assert_eq!(emulator.generic_registers[0xB], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 12);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_ANNN() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0xAF, 0xEB]);
        emulator.process_next_instruction();
        assert_eq!(emulator.memory_register, 0x0FEB);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_CXNN() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0xC0, 0x10]);

        emulator.generic_registers[0x0] = 0x11;
        emulator.process_next_instruction();
        assert_ne!(emulator.generic_registers[0x0], 0x11);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_DXYN() {
        let mut emulator: Emulator = Emulator::new();
        emulator.load_program(&[0xD0, 0x12, 0xD0, 0x12]);

        // Cheating a bit for a faster setup
        emulator.generic_registers[0] = 0x05;
        emulator.generic_registers[1] = 0x06;
        emulator.generic_registers[0xF] = 0x10;
        emulator.memory_register = 0x300;
        emulator.memory[0x300] = 0b10101010;
        emulator.memory[0x301] = 0b11001100;

        emulator.process_next_instruction();

        for j in 0..CHIP8_SCREEN_HEIGHT {
            println!(
                "{:?}",
                &emulator.screen[j * CHIP8_SCREEN_WIDTH..(j + 1) * CHIP8_SCREEN_WIDTH]
            );
        }

        assert_eq!(
            emulator.screen[0x05 + 0x06 * CHIP8_SCREEN_WIDTH + 0],
            PixelStatus::White
        );
        assert_eq!(
            emulator.screen[0x05 + 0x06 * CHIP8_SCREEN_WIDTH + 1],
            PixelStatus::Black
        );
        assert_eq!(
            emulator.screen[0x05 + 0x06 * CHIP8_SCREEN_WIDTH + 2],
            PixelStatus::White
        );
        assert_eq!(
            emulator.screen[0x05 + 0x06 * CHIP8_SCREEN_WIDTH + 3],
            PixelStatus::Black
        );
        assert_eq!(
            emulator.screen[0x05 + 0x06 * CHIP8_SCREEN_WIDTH + 4],
            PixelStatus::White
        );
        assert_eq!(
            emulator.screen[0x05 + 0x06 * CHIP8_SCREEN_WIDTH + 5],
            PixelStatus::Black
        );
        assert_eq!(
            emulator.screen[0x05 + 0x06 * CHIP8_SCREEN_WIDTH + 6],
            PixelStatus::White
        );
        assert_eq!(
            emulator.screen[0x05 + 0x06 * CHIP8_SCREEN_WIDTH + 7],
            PixelStatus::Black
        );

        assert_eq!(
            emulator.screen[0x05 + 0x07 * CHIP8_SCREEN_WIDTH + 0],
            PixelStatus::White
        );
        assert_eq!(
            emulator.screen[0x05 + 0x07 * CHIP8_SCREEN_WIDTH + 1],
            PixelStatus::White
        );
        assert_eq!(
            emulator.screen[0x05 + 0x07 * CHIP8_SCREEN_WIDTH + 2],
            PixelStatus::Black
        );
        assert_eq!(
            emulator.screen[0x05 + 0x07 * CHIP8_SCREEN_WIDTH + 3],
            PixelStatus::Black
        );
        assert_eq!(
            emulator.screen[0x05 + 0x07 * CHIP8_SCREEN_WIDTH + 4],
            PixelStatus::White
        );
        assert_eq!(
            emulator.screen[0x05 + 0x07 * CHIP8_SCREEN_WIDTH + 5],
            PixelStatus::White
        );
        assert_eq!(
            emulator.screen[0x05 + 0x07 * CHIP8_SCREEN_WIDTH + 6],
            PixelStatus::Black
        );
        assert_eq!(
            emulator.screen[0x05 + 0x07 * CHIP8_SCREEN_WIDTH + 7],
            PixelStatus::Black
        );

        assert_eq!(emulator.generic_registers[0xF], 0x0);

        // Calling it again, all pixels should be toggled back to black + VF set to 0x01
        emulator.process_next_instruction();
        for i in 0..8 {
            assert_eq!(
                emulator.screen[0x05 + 0x06 * CHIP8_SCREEN_WIDTH + i],
                PixelStatus::Black
            );
            assert_eq!(
                emulator.screen[0x05 + 0x07 * CHIP8_SCREEN_WIDTH + i],
                PixelStatus::Black
            );
        }
        assert_eq!(emulator.generic_registers[0xF], 0x1)

    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_EX9E_EXA1() {
        let mut emulator: Emulator = Emulator::new();
        emulator.load_program(&[0xE0, 0x9E, 0xE0, 0x9E, 0x00, 0x00, 0xE0, 0xA1, 0xE0, 0xA1]);

        emulator.generic_registers[0x0] = 0x01;
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2);

        emulator.keys_pressed[0x01] = true;
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);

        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);

        emulator.keys_pressed[0x01] = false;
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 12);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_FX07_FX15() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0xF0, 0x15, 0xF0, 0x07]);

        emulator.generic_registers[0x00] = 0x16;
        emulator.process_next_instruction();
        assert_eq!(emulator.system_clock, 0x16);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2);

        emulator.system_clock -= 5;
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0x00], 0x11);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_FX18() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0xF0, 0x18]);

        emulator.generic_registers[0x00] = 0x16;
        emulator.process_next_instruction();
        assert_eq!(emulator.sound_clock, 0x16);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_FX1E() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x60, 0xFF, 0x6F, 0x10, 0xAF, 0xFF, 0xF0, 0x1E, 0xF0, 0x1E]);

        emulator.process_next_instruction();
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0x0], 0xFF);
        assert_eq!(emulator.generic_registers[0xF], 0x10);
        assert_eq!(emulator.memory_register, 0xFFF);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);

        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0x0], 0xFF);
        assert_eq!(emulator.generic_registers[0xF], 0x01);
        assert_eq!(emulator.memory_register, 0xFF);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);

        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0x0], 0xFF);
        assert_eq!(emulator.generic_registers[0xF], 0x00);
        assert_eq!(emulator.memory_register, 0x1FE);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 10);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_FX55() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0x60, 0b10101010, 0x61, 0b00110011, 0xA1, 0x55, 0xF1, 0x55]);

        emulator.process_next_instruction();
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0x0], 0b10101010);
        assert_eq!(emulator.generic_registers[0x1], 0b00110011);
        assert_eq!(emulator.memory_register, 0x155);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 6);

        emulator.process_next_instruction();
        assert_eq!(emulator.memory[0x155], 0b10101010);
        assert_eq!(emulator.memory[0x156], 0b00110011);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_FX65() {
        let mut emulator = Emulator::new();
        emulator.load_program(&[0xF1, 0x65]);

        // Cheating a bit with the setup to go faster
        emulator.memory[0xF00] = 0b10101010;
        emulator.memory[0xF01] = 0b11001100;
        emulator.memory_register = 0xF00;

        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0x0], 0b10101010);
        assert_eq!(emulator.generic_registers[0x1], 0b11001100);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2);
    }
}

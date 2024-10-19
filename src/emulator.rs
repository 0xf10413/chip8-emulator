const CHIP8_MEMORY_SIZE: usize = 4096;
const CHIP8_FIRST_BYTE_ADDRESS: usize = 512;
const CHIP8_NUMBER_REGISTERS: usize = 16;
//const CHIP8_CALL_STACK_SIZE: usize = 16;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum OpCode {
    OC_0NNN(u16),
    // 00E0 => missing
    // 00EE => missing
    OC_1NNN(usize),
    // 2NNN => missing
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
}

fn parse_opcode(raw_opcode: u16) -> Option<OpCode> {
    // 0NNN
    if raw_opcode & 0x0FFF == 0x0000 {
        let nnn: u16 = raw_opcode & 0x0FFF;
        return Some(OpCode::OC_0NNN(nnn));
    }

    // 1NNN
    if raw_opcode & 0xF000 == 0x1000 {
        let nnn: usize = (0x0FFF & raw_opcode) as usize;
        return Some(OpCode::OC_1NNN(nnn));
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

    return None;
}

#[derive(Debug)]
pub struct EmulatorCpuMemory {
    memory: [u8; CHIP8_MEMORY_SIZE],
    program_counter: usize,
    generic_registers: [u8; CHIP8_NUMBER_REGISTERS],
    //opcode_register: u8,
    //call_stack: [usize; CHIP8_CALL_STACK_SIZE], // TODO: make an actual stack struct
    //call_stack_index: usize, // TODO: make an actual stack struct
}

impl EmulatorCpuMemory {
    pub fn new() -> Self {
        Self {
            memory: [0; CHIP8_MEMORY_SIZE],
            program_counter: CHIP8_FIRST_BYTE_ADDRESS,
            generic_registers: [0; CHIP8_NUMBER_REGISTERS],
            //opcode_register: 0,
            //call_stack: [0; CHIP8_CALL_STACK_SIZE],
            //call_stack_index: 0,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.memory[CHIP8_FIRST_BYTE_ADDRESS+i] = *byte
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
            OpCode::OC_0NNN(_) => {
                // Calls code routine at address NNN
                panic!("OpCode 0NNN not implemented!")
            }

            OpCode::OC_1NNN(nnn) => {
                // Next instruction will be at address NN
                println!("Setting pc to {}", nnn);
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
                }            }

            OpCode::OC_6XNN(x, nn) => {
                // Defines register VX to NN
                println!("Setting register V{:X} to {:#X}", x, nn);
                self.generic_registers[*x] = *nn;
            }

            OpCode::OC_7XNN(x, nn) => {
                // Adds NN to register VX
                println!("Adding {:#X} to register V{:X}", nn, x);
                self.generic_registers[*x] += *nn;
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
                println!("Adding register V{:X} with the value of V{:X}, while putting carry in VF", x, y);
                let (result, overflow ) = self.generic_registers[*x].overflowing_add(self.generic_registers[*y]);
                self.generic_registers[*x] = result;
                self.generic_registers[0xF] = if overflow { 1 } else { 0 };
            }

            OpCode::OC_8XY5(x, y) => {
                // Set register VX to the value of VX - VY, write carry in VF
                println!("Substracting register V{:X} with the value of V{:X}, while putting carry in VF", x, y);
                let (result, overflow ) = self.generic_registers[*x].overflowing_sub(self.generic_registers[*y]);
                self.generic_registers[*x] = result;
                self.generic_registers[0xF] = if overflow { 1 } else { 0 };
            }

            OpCode::OC_8XY6(x, ..) => {
                // Shifts VX to the right by 1 bit. VF will contain the lost bit.
                // Somehow, Y is not used?
                println!("Shifting right register V{:X} with the lost bit written in VF", x);
                self.generic_registers[0xF] = self.generic_registers[*x] & 0x01;
                self.generic_registers[*x] >>= 1;
            }

            OpCode::OC_8XY7(x, y) => {
                // Sets VX to VY - VX. VF is set to 0 if there is an overflow, 1 otherwise.
                println!("Setting register V{:X} to V{:X} - V{:X} with opposite of overflow written in VF", x, y, x);
                let (result, overflow ) = self.generic_registers[*y].overflowing_sub(self.generic_registers[*x]);
                self.generic_registers[*x] = result;
                self.generic_registers[0xF] = if overflow { 0 } else { 1 };
            }

            OpCode::OC_8XYE(x, ..) => {
                // Shifts VX to the left by 1 bit. VF will contain the lost bit.
                // Somehow, Y is not used?
                println!("Shifting left register V{:X} with the lost bit written in VF", x);
                self.generic_registers[0xF] = (self.generic_registers[*x] & 0b10000000) >> 7;
                self.generic_registers[*x] <<= 1;
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
        let mut emulator = EmulatorCpuMemory::new();
        emulator.load_program(&[0x80, 0x0F]);
        emulator.process_next_instruction();
    }

    #[test]
    #[allow(non_snake_case)]
    #[should_panic(expected = "OpCode 0NNN not implemented!")]
    fn test_opcode_0NNN() {
        let mut emulator = EmulatorCpuMemory::new();
        emulator.load_program(&[0x00, 0x00]);
        emulator.process_next_instruction();
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_1NNN() {
        let mut emulator = EmulatorCpuMemory::new();
        emulator.load_program(&[0x12, 0xFF]);
        emulator.process_next_instruction();
        assert_eq!(emulator.program_counter, 0x2FF);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_3XNN() {
        let mut emulator = EmulatorCpuMemory::new();
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
        let mut emulator = EmulatorCpuMemory::new();
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
        let mut emulator = EmulatorCpuMemory::new();
        emulator.load_program(&[0x6A, 0x01, 0x6B, 0x01, 0x5A, 0xB0, 0x6A, 0x10, 0x6A, 0x02, 0x5A, 0xB0]);
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
        let mut emulator = EmulatorCpuMemory::new();
        emulator.load_program(&[0x6A, 0x15]);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0x15);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_7XNN() {
        let mut emulator = EmulatorCpuMemory::new();
        emulator.load_program(&[0x7B, 0x03, 0x7B, 0x05]);
        emulator.process_next_instruction();
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xB], 0x08);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 4);
    }


    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XY0() {
        let mut emulator = EmulatorCpuMemory::new();
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
        let mut emulator = EmulatorCpuMemory::new();
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
        let mut emulator = EmulatorCpuMemory::new();
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
        let mut emulator = EmulatorCpuMemory::new();
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
        let mut emulator = EmulatorCpuMemory::new();
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
        let mut emulator = EmulatorCpuMemory::new();
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
        assert_eq!(emulator.generic_registers[0xF], 0x00);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 8);
        emulator.process_next_instruction();
        assert_eq!(emulator.generic_registers[0xA], 0xFF);
        assert_eq!(emulator.generic_registers[0xB], 0x02);
        assert_eq!(emulator.generic_registers[0xF], 0x01);
        assert_eq!(emulator.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 10);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_8XY6() {
        let mut emulator = EmulatorCpuMemory::new();
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
        let mut emulator = EmulatorCpuMemory::new();
        emulator.load_program(&[0x6A, 0x04, 0x6B, 0x03, 0x6F, 0x10, 0x8A, 0xB7, 0x6A, 0x01, 0x8A, 0xB7]);
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
        let mut emulator = EmulatorCpuMemory::new();
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
}

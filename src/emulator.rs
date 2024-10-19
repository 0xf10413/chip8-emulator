const CHIP8_MEMORY_SIZE: usize = 4096;
pub const CHIP8_FIRST_BYTE_ADDRESS: usize = 512;
const CHIP8_NUMBER_REGISTERS: usize = 16;
//const CHIP8_CALL_STACK_SIZE: usize = 16;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum OpCode {
    OC_0NNN(u16),
    OC_6XNN(usize, u8),
}

fn parse_opcode(raw_opcode: u16) -> Option<OpCode> {
    // 0NNN
    if raw_opcode & 0x0FFF == 0x0000 {
        let nnn: u16 = raw_opcode & 0x0FFF;
        return Some(OpCode::OC_0NNN(nnn));
    }

    // 6XNN
    if raw_opcode & 0xF000 == 0x6000 {
        let x: usize = ((0x0F00 & raw_opcode) >> 8) as usize;
        let nn: u8 = (0x00FF & raw_opcode) as u8;
        return Some(OpCode::OC_6XNN(x, nn));
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
            None => panic!("Unidentified opcode!"),
        }

        // Jump to next instruction
        self.program_counter += 2;
    }

    fn process_opcode(&mut self, opcode: &OpCode) {
        match opcode {
            OpCode::OC_0NNN(_) => {
                // 0NNN: Calls code routine at address NNN
                panic!("OpCode 0NNN not implemented!")
            }

            OpCode::OC_6XNN(x, nn) => {
                // 6XNN: Defines register VX to NN
                println!("Setting register V{:X} to {:#X}", x, nn);
                self.generic_registers[*x] = *nn;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_6XNN() {
        let mut state = EmulatorCpuMemory::new();
        state.memory[CHIP8_FIRST_BYTE_ADDRESS] = 0x6A;
        state.memory[CHIP8_FIRST_BYTE_ADDRESS + 1] = 0x15;
        state.process_next_instruction();
        assert_eq!(state.generic_registers[0xA], 0x15);
        assert_eq!(state.program_counter, CHIP8_FIRST_BYTE_ADDRESS + 2)
    }
}

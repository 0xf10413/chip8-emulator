const CHIP8_MEMORY_SIZE: usize = 4096;
const CHIP8_FIRST_BYTE_ADDRESS: usize = 512;
const CHIP8_NUMBER_REGISTERS: usize = 16;
//const CHIP8_CALL_STACK_SIZE: usize = 16;

#[derive(Debug)]
struct OpCode<'a> {
    id: u16,
    mask: u16,
    name: &'a str,
    update_state: fn(&mut EmulatorCpuMemory, u16) -> (),
}

impl OpCode<'_> {
    fn identify(&self, read_opcode: u16) -> bool {
        println!(
            "{}: {:#06x} & {:#06x} = {:#06x} vs {:#06x}",
            self.name,
            read_opcode,
            self.mask,
            read_opcode & self.mask,
            self.id
        );
        return read_opcode & self.mask == self.id;
    }
}

fn unimplemented_opcode(_emulator: &mut EmulatorCpuMemory, _read_opcode: u16) {
    panic!("Opcode is not implemented yet!")
}

const OPCODES: [OpCode; 9] = [
    OpCode {
        id: 0x0FFF,
        mask: 0x000,
        name: "0NNN",
        update_state: unimplemented_opcode,
    },
    OpCode {
        id: 0x00E0,
        mask: 0xFFFF,
        name: "00E0",
        update_state: unimplemented_opcode,
    },
    OpCode {
        id: 0x00EE,
        mask: 0xFFFF,
        name: "00EE",
        update_state: unimplemented_opcode,
    },
    OpCode {
        id: 0x1000,
        mask: 0xF000,
        name: "1NNN",
        update_state: unimplemented_opcode,
    },
    OpCode {
        id: 0x2000,
        mask: 0xF000,
        name: "2NNN",
        update_state: unimplemented_opcode,
    },
    OpCode {
        id: 0x3000,
        mask: 0xF000,
        name: "3XNN",
        update_state: unimplemented_opcode,
    },
    OpCode {
        id: 0x4000,
        mask: 0xF000,
        name: "4XNN",
        update_state: unimplemented_opcode,
    },
    OpCode {
        id: 0x5000,
        mask: 0xF00F,
        name: "5XY0",
        update_state: unimplemented_opcode,
    },
    OpCode {
        id: 0x6000,
        mask: 0xF000,
        name: "6XNN",
        update_state: |emulator, read_opcode| {
            // 6XNN: Defines register VX to NN
            let x: usize = ((0x0F00 & read_opcode) >> 8).into();
            let nn: u8 = 0x00FF & read_opcode as u8;
            println!("6XNN: Found X = {:#03x} and NN = {:#04x}", x, nn);
            println!("Setting register V{:X} to {:#X}", x, nn);
            emulator.generic_registers[x] = nn;
        },
    },
];

#[derive(Debug)]
struct EmulatorCpuMemory {
    memory: [u8; CHIP8_MEMORY_SIZE],
    program_counter: usize,
    generic_registers: [u8; CHIP8_NUMBER_REGISTERS],
    //opcode_register: u8,
    //call_stack: [usize; CHIP8_CALL_STACK_SIZE],
    //call_stack_index: usize, // TODO: make an actual stack
}

impl EmulatorCpuMemory {
    fn new() -> Self {
        Self {
            memory: [0; CHIP8_MEMORY_SIZE],
            program_counter: CHIP8_FIRST_BYTE_ADDRESS,
            generic_registers: [0; CHIP8_NUMBER_REGISTERS],
            //opcode_register: 0,
            //call_stack: [0; CHIP8_CALL_STACK_SIZE],
            //call_stack_index: 0,
        }
    }

    fn process_next_instruction(&mut self) {
        println!("Reading code and processing next instruction...");

        // Read next, which is build from the next two bytes
        let opcode_first_part: u16 = self.memory[self.program_counter] as u16;
        let opcode_second_part: u16 = self.memory[self.program_counter + 1] as u16;
        let opcode_raw: u16 = (opcode_first_part << 8) + opcode_second_part;
        println!("Opcode read: {:#06x}", opcode_raw);

        let identified_opcode = OPCODES.iter().find(|&x| x.identify(opcode_raw));

        match identified_opcode {
            Some(opcode) => println!("Identified read opcode as {}", opcode.name),
            None => panic!("Unidentified opcode!"),
        }

        (identified_opcode.unwrap().update_state)(self, opcode_raw);
    }
}

fn main() {
    let mut state = EmulatorCpuMemory::new();
    state.memory[CHIP8_FIRST_BYTE_ADDRESS] = 0x6A;
    state.memory[CHIP8_FIRST_BYTE_ADDRESS + 1] = 0x10;
    state.process_next_instruction();
    //println!("{:#?}", state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn test_opcode_6XNN() {
        let mut state = EmulatorCpuMemory::new();
        state.memory[CHIP8_FIRST_BYTE_ADDRESS] = 0x6A;
        state.memory[CHIP8_FIRST_BYTE_ADDRESS + 1] = 0x10;
        state.process_next_instruction();    
        assert_eq!(state.generic_registers[0xA], 0x10);
    }
}

struct CPU {
    position_in_memory: usize,
    registers: [u8; 16],
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            position_in_memory: 0,
            registers: [0; 16],
            memory: [0; 4096],
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;
            let nnn = opcode & 0x0FFF;
            println!("c: {:X}, x: {:X}, y:{:X}, d: {:X}", c, x, y, d);

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }

    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow_detected {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn load_opcode(&mut self, opcode: u16, at: u16) {
        if at as usize > self.memory.len() - 1 {
            panic!("Cannot load overflolwing opcode");
        }

        let op_byte_1 = (opcode >> 8) as u8;
        let op_byte_2 = (opcode & 0xFF) as u8;

        self.memory[at as usize] = op_byte_1;
        self.memory[(at + 1) as usize] = op_byte_2;
    }
}

struct Opcode {}

impl Opcode {
    fn call(addr: u16) -> u16 {
        0x2000 | addr
    }

    fn ret() -> u16 {
        0x00EE
    }

    fn add(x: u16, y: u16) -> u16 {
        0x8000 | (x << 4 | y) << 4 | 0x0004
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.load_opcode(Opcode::call(0x100), 0);
    cpu.load_opcode(Opcode::call(0x100), 2);

    cpu.load_opcode(Opcode::add(0, 1), 0x100);
    cpu.load_opcode(Opcode::add(0, 1), 0x102);
    cpu.load_opcode(Opcode::ret(), 0x104);
    cpu.run();
    assert_eq!(cpu.registers[0], 45);
    println!("Result in cpu: {}", cpu.registers[0])
}

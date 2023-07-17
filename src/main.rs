struct BitState {
    pc: u32,
    registers: [u32; 256],
}

impl BitState {
    fn new() -> Self {
        BitState {
            pc: 0,
            registers: [0; 256],
        }
    }

    fn exec_instr(&mut self, instr: &Instr) {
        self.pc += 1;
        match instr {
            Instr::Nop => {}
            Instr::Store(value, register) => {
                self.registers[usize::from(*register)] = *value;
            }
            Instr::Add(left_reg, right_reg, dest_reg) => {
                let left = self.registers[usize::from(*left_reg)];
                let right: u32 = self.registers[usize::from(*right_reg)];
                self.registers[usize::from(*dest_reg)] = left + right;
            }
        }
    }

    fn show(&self) {
        println!("registers: {:?}", self.registers);
        println!("pc: {:?}\n", self.pc);
    }
}

enum Instr {
    Nop,
    Store(u32, u8),
    Add(u8, u8, u8),
}

fn run_code<TCode>(code: TCode)
where
    TCode: Iterator<Item = Instr>,
{
    let mut state = BitState::new();

    for instr in code {
        state.exec_instr(&instr);
        state.show();
    }
}

fn main() {
    println!("This is BitVM");

    let code = vec![
        Instr::Store(1, 10),
        Instr::Store(2, 11),
        Instr::Add(10, 11, 12),
        Instr::Nop,
    ];

    run_code(code.into_iter());
}

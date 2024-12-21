use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let l: Vec<&str> = content.split("\n").collect();
    let text = l[4]
        .split_once(": ")
        .ok_or("Parsing Err")?
        .1
        .split(",")
        .map(|n| n.parse().expect("Parsing Err"))
        .collect();
    let mut program = Machine {
        pc: 0,
        a: l[0].split_once(": ").ok_or("Parsing Err")?.1.parse()?,
        b: l[1].split_once(": ").ok_or("Parsing Err")?.1.parse()?,
        c: l[2].split_once(": ").ok_or("Parsing Err")?.1.parse()?,
        text,
        output: vec![],
    };

    while let Some(_) = program.step() {}
    println!("{}", program.output_str());

    Ok(())
}

#[derive(Debug, Default)]
struct Machine {
    pc: usize,
    a: usize,
    b: usize,
    c: usize,
    text: Vec<u8>,
    output: Vec<u8>,
}

#[repr(u8)]
#[derive(Clone)]
enum Instr {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl TryFrom<u8> for Instr {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const INSTRS: [Instr; 8] = [
            Instr::ADV,
            Instr::BXL,
            Instr::BST,
            Instr::JNZ,
            Instr::BXC,
            Instr::OUT,
            Instr::BDV,
            Instr::CDV,
        ];
        INSTRS.get(value as usize).cloned().ok_or(())
    }
}

impl Machine {
    fn step(&mut self) -> Option<()> {
        let instr: Instr = Instr::try_from(*self.text.get(self.pc)?).ok()?;
        let operand: u8 = *self.text.get(self.pc + 1)?;
        self.pc += 2;

        match instr {
            Instr::ADV => self.a = self.a / (2_usize.pow(self.combo_operand(operand) as u32)),
            Instr::BXL => self.b = self.b ^ operand as usize,
            Instr::BST => self.b = self.combo_operand(operand) % 8,
            Instr::JNZ => {
                if self.a != 0 {
                    self.pc = operand as usize
                }
            }
            Instr::BXC => self.b = self.b ^ self.c,
            Instr::OUT => self.output.push((self.combo_operand(operand) % 8) as u8),
            Instr::BDV => self.b = self.a / (2_usize.pow(self.combo_operand(operand) as u32)),
            Instr::CDV => self.c = self.a / (2_usize.pow(self.combo_operand(operand) as u32)),
        };

        Some(())
    }

    fn combo_operand(&self, operand: u8) -> usize {
        match operand {
            0..=3 => operand.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Combo Operand 7 is reserved"),
            _ => unreachable!(),
        }
    }

    fn output_str(&self) -> String {
        self.output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

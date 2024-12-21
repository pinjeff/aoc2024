use std::{error::Error, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let l: Vec<&str> = content.split("\n").collect();
    let text: Vec<u8> = l[4]
        .split_once(": ")
        .ok_or("Parsing Err")?
        .1
        .split(",")
        .map(|n| n.parse().expect("Parsing Err"))
        .collect();

    let mut queue = vec![(text.len() - 1, 0)];
    let mut answers = vec![];
    while let Some((offset, a)) = queue.pop() {
        for i in 0..8 {
            if compute(a * 8 + i, &text)[..] == text[offset..] {
                if offset == 0 {
                    answers.push(a * 8 + i);
                    break;
                }
                queue.insert(0, (offset - 1, a * 8 + i));
            }
        }
    }

    answers.sort();
    let answer = answers.first().ok_or("Could not find a answer")?;
    println!("Value of A to get a quine: {}", answer);
    Ok(())
}

fn compute(a: usize, text: &[u8]) -> Vec<u8> {
    let mut p = Machine {
        pc: 0,
        a,
        b: 0,
        c: 0,
        text,
        output: vec![],
    };

    while let Some(_) = p.step() {}

    p.output
}

#[derive(Debug)]
struct Machine<'a> {
    pc: usize,
    a: usize,
    b: usize,
    c: usize,
    text: &'a [u8],
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

impl<'a> Machine<'a> {
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
}

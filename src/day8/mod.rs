use crate::Solution;
use std::collections::HashSet;
use std::error::Error;

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let instructions = input
            .lines()
            .map(Instruction::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self { instructions })
    }
}

struct Instruction {
    kind: InstructionKind,
    value: i16,
}

impl Instruction {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut parts = input.split_whitespace();
        let kind = InstructionKind::parse(parts.next().ok_or("Missing instruction")?)?;
        let value = parts.next().ok_or("Missing value")?.parse()?;
        Ok(Instruction { kind, value })
    }
}

#[derive(Copy, Clone)]
enum InstructionKind {
    Acc,
    Jmp,
    Nop,
}

impl InstructionKind {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        Ok(match input {
            "acc" => InstructionKind::Acc,
            "jmp" => InstructionKind::Jmp,
            "nop" => InstructionKind::Nop,
            _ => return Err("Unrecognized instruction".into()),
        })
    }
}

struct Execution<'a> {
    acc: i16,
    pc: i16,
    instructions: &'a [Instruction],
}

impl<'a> Execution<'a> {
    fn new(program: &'a Program) -> Self {
        Self {
            acc: 0,
            pc: 0,
            instructions: &program.instructions,
        }
    }

    fn step(&mut self) -> bool {
        if let Some(Instruction { kind, value }) = self.instructions.get(self.pc as usize) {
            match kind {
                InstructionKind::Acc => self.acc += value,
                InstructionKind::Jmp => {
                    self.pc += value;
                    return true;
                }
                InstructionKind::Nop => {}
            }
            self.pc += 1;
            true
        } else {
            false
        }
    }
}

pub(super) const DAY8: Solution = Solution {
    part1: |input| {
        let program = Program::parse(input)?;
        let mut execution = Execution::new(&program);
        let mut encountered = HashSet::new();
        while execution.step() {
            if !encountered.insert(execution.pc) {
                return Ok(execution.acc.to_string());
            }
        }
        Err("The program exited successfully".into())
    },
    part2: |input| {
        let mut program = Program::parse(input)?;
        'search_loop: for instruction in 0..program.instructions.len() {
            let instruction_kind = &mut program.instructions[instruction].kind;
            let original_kind = *instruction_kind;
            *instruction_kind = match instruction_kind {
                InstructionKind::Jmp => InstructionKind::Nop,
                InstructionKind::Nop => InstructionKind::Jmp,
                _ => continue,
            };
            let mut execution = Execution::new(&program);
            let mut encountered = HashSet::new();
            while execution.step() {
                if !encountered.insert(execution.pc) {
                    program.instructions[instruction].kind = original_kind;
                    continue 'search_loop;
                }
            }
            return Ok(execution.acc.to_string());
        }
        Err("No valid solution was found".into())
    },
};

#[cfg(test)]
mod test {
    use crate::{lines, test};
    const EXAMPLE: &str = lines!(
        "nop +0"
        "acc +1"
        "jmp +4"
        "acc +3"
        "jmp -3"
        "acc -99"
        "acc +1"
        "jmp -4"
        "acc +6"
    );
    test!(
        DAY8.part1,
        example: EXAMPLE => 5,
        input: 1420,
    );
    test!(
        DAY8.part2,
        example: EXAMPLE => 8,
        input: 1245,
    );
}

use crate::{Instruction, RuntimeError};
use std::io::{Cursor, Read, Write};

/// A simple Brainfuck virtual machine.
///
/// It contains:
/// - A fixed-size memory tape with 30,000 cells.
/// - A pointer to the current active cell.
///
/// Each memory cell stores an unsigned 8-bit integer (`u8`).
pub struct Machine {
    // The memory of a typical Brainfuck machine has 30,000 memory
    // cells; each holding a value from 0 to 255.
    tape: [u8; 30_000],

    // Points to the current memory cell index.
    ptr: usize,
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            tape: [0; 30_000],
            ptr: 0,
        }
    }
}

impl Default for Machine {
    fn default() -> Self {
        Machine::new()
    }
}

impl Machine {
    pub fn inc(&mut self) {
        self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(1);
    }

    pub fn dec(&mut self) {
        self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(1);
    }

    pub fn current(&self) -> u8 {
        self.tape[self.ptr]
    }

    pub fn set_current(&mut self, value: u8) {
        self.tape[self.ptr] = value;
    }

    pub fn move_right(&mut self) -> Result<(), RuntimeError> {
        if self.ptr + 1 >= self.tape.len() {
            return Err(RuntimeError::PointerOutOfBounds);
        }

        self.ptr += 1;
        Ok(())
    }

    pub fn move_left(&mut self) -> Result<(), RuntimeError> {
        if self.ptr == 0 {
            return Err(RuntimeError::PointerOutOfBounds);
        }

        self.ptr -= 1;
        Ok(())
    }

    pub fn output(&self, writer: &mut impl Write) -> Result<(), RuntimeError> {
        writer
            .write_all(&[self.current()])
            .map_err(|_| RuntimeError::IoError)
    }

    pub fn read(&mut self, reader: &mut impl Read) -> Result<(), RuntimeError> {
        let mut buf = [0u8, 1];
        match reader.read(&mut buf) {
            Ok(0) => Ok(()), // EOF
            Ok(_) => {
                self.set_current(buf[0]);
                Ok(())
            }
            Err(_) => Err(RuntimeError::IoError),
        }
    }
}

/// Executes a parsed Brainfuck program.
pub fn run<R: Read, W: Write>(
    machine: &mut Machine,
    program: &[Instruction],
    input: &mut R,
    output: &mut W,
) -> Result<(), RuntimeError> {
    // `pc` stands for "program counter" and tracks which instruction
    // is going be executed. Since the instructions are already resolved,
    // pc is just an index into `program`.
    let mut pc = 0;

    while pc < program.len() {
        match program[pc] {
            Instruction::MoveRight => machine.move_right()?,
            Instruction::MoveLeft => machine.move_left()?,
            Instruction::Inc => machine.inc(),
            Instruction::Dec => machine.dec(),
            Instruction::Write => machine.output(output)?,
            Instruction::Read => machine.read(input)?,
            Instruction::JumpIfZero(target) => {
                if machine.current() == 0 {
                    pc = target;
                    continue;
                }
            }
            Instruction::JumpIfNonZero(target) => {
                if machine.current() != 0 {
                    pc = target;
                    continue;
                }
            }
        }

        pc += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_machine_starts_at_zero() {
        let m = Machine::new();
        assert_eq!(m.current(), 0);
    }

    #[test]
    fn dec_wraps_on_overflow() {
        let mut m = Machine::new();
        m.dec();
        assert_eq!(m.current(), 255);
    }

    #[test]
    fn inc_wraps_on_overflow() {
        let mut m = Machine::new();
        m.set_current(255);
        m.inc();
        assert_eq!(m.current(), 0);
    }

    #[test]
    fn move_right_increments_ptr() {
        let mut m = Machine::new();
        m.move_right().unwrap();
        assert_eq!(m.ptr, 1);
    }

    #[test]
    fn move_left_decrements_ptr() {
        let mut m = Machine::new();
        m.move_right().unwrap();
        m.move_left().unwrap();
        assert_eq!(m.ptr, 0);
    }

    #[test]
    fn move_left_at_zero_errors() {
        let mut m = Machine::new();
        let result = m.move_left();
        assert_eq!(result, Err(RuntimeError::PointerOutOfBounds));
        assert_eq!(m.ptr, 0);
    }

    #[test]
    fn move_right_at_last_cell_errors() {
        let mut m = Machine::new();
        m.ptr = 29_999;
        let result = m.move_right();
        assert_eq!(result, Err(RuntimeError::PointerOutOfBounds));
        assert_eq!(m.ptr, 29_999);
    }

    #[test]
    fn read_reads_byte_into_current_cell() {
        let mut m = Machine::new();
        let mut input = Cursor::new(vec![65u8]); // 'A'
        m.read(&mut input).unwrap();
        assert_eq!(m.current(), 65);
    }

    #[test]
    fn output_writes_current_cell() {
        let mut m = Machine::new();
        m.set_current(65);
        let mut output = Vec::new();
        m.output(&mut output).unwrap();
        assert_eq!(output, vec![65]);
    }
}

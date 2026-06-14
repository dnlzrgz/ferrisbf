use crate::{Instruction, RuntimeError};
use std::io::{Read, Write};

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
    pub fn add(&mut self, n: u8) {
        let cell = &mut self.tape[self.ptr];
        *cell = cell.wrapping_add(n);
    }

    pub fn shift(&mut self, n: isize) -> Result<(), RuntimeError> {
        let new_ptr = self.ptr as isize + n;
        if new_ptr < 0 || new_ptr as usize >= self.tape.len() {
            return Err(RuntimeError::PointerOutOfBounds);
        }
        self.ptr = new_ptr as usize;
        Ok(())
    }

    pub fn current(&self) -> u8 {
        self.tape[self.ptr]
    }

    pub fn set_current(&mut self, value: u8) {
        self.tape[self.ptr] = value;
    }

    pub fn output(&self, writer: &mut impl Write) -> Result<(), RuntimeError> {
        writer
            .write_all(&[self.current()])
            .map_err(|_| RuntimeError::IoError)
    }

    pub fn read(&mut self, reader: &mut impl Read) -> Result<(), RuntimeError> {
        let mut buf = [0u8; 1];
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
            Instruction::Add(n) => machine.add(n),
            Instruction::Move(n) => machine.shift(n)?,
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
    use std::io::Cursor;

    #[test]
    fn new_machine_starts_at_zero() {
        let m = Machine::new();
        assert_eq!(m.current(), 0);
    }

    #[test]
    fn add_wraps_on_overflow_down() {
        let mut m = Machine::new();
        m.add(255);
        assert_eq!(m.current(), 255);
    }

    #[test]
    fn add_wraps_on_overflow_up() {
        let mut m = Machine::new();
        m.set_current(255);
        m.add(1);
        assert_eq!(m.current(), 0);
    }

    #[test]
    fn shift_right_increments_ptr() {
        let mut m = Machine::new();
        m.shift(1).unwrap();
        assert_eq!(m.ptr, 1);
    }

    #[test]
    fn shift_left_decrements_ptr() {
        let mut m = Machine::new();
        m.shift(1).unwrap();
        m.shift(-1).unwrap();
        assert_eq!(m.ptr, 0);
    }

    #[test]
    fn shift_left_ar_zero_errors() {
        let mut m = Machine::new();
        let result = m.shift(-1);
        assert_eq!(result, Err(RuntimeError::PointerOutOfBounds));
        assert_eq!(m.ptr, 0);
    }

    #[test]
    fn shift_right_at_last_cell_errors() {
        let mut m = Machine::new();
        m.ptr = 29_999;
        let result = m.shift(1);
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

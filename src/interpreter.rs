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

    pub fn move_right(&mut self) {
        unimplemented!();
    }

    pub fn move_left(&mut self) {
        unimplemented!();
    }
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
}

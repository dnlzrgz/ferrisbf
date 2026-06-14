use crate::error::ParseError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Move(isize),          // <,>
    Add(u8),              // +,-
    Write,                // .
    Read,                 // ,
    JumpIfZero(usize),    // [
    JumpIfNonZero(usize), // ]
}

/// Parses Brainfunc source code into a sequence of instructions.
pub fn parse(source: &str) -> Result<Vec<Instruction>, ParseError> {
    let bytes = source.as_bytes();
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut bracket_stack: Vec<usize> = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'>' | b'<' => {
                let mut net = 0;
                while i < bytes.len() && (bytes[i] == b'>' || bytes[i] == b'<') {
                    net += if bytes[i] == b'>' { 1 } else { -1 };
                    i += 1;
                }
                if net != 0 {
                    instructions.push(Instruction::Move(net));
                }
            }
            b'+' | b'-' => {
                let mut net: i32 = 0;
                while i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
                    net += if bytes[i] == b'+' { 1 } else { -1 };
                    i += 1;
                }

                let amount = net.rem_euclid(256) as u8;
                if amount != 0 {
                    instructions.push(Instruction::Add(amount));
                }
            }
            b'.' => {
                instructions.push(Instruction::Write);
                i += 1;
            }
            b',' => {
                instructions.push(Instruction::Read);
                i += 1;
            }
            b'[' => {
                // target is known at this time, so we use 0.
                instructions.push(Instruction::JumpIfZero(0));
                bracket_stack.push(instructions.len() - 1);
                i += 1;
            }
            b']' => {
                let open_index = bracket_stack
                    .pop()
                    .ok_or(ParseError::UnmatchedCloseBracket)?;
                let close_index = instructions.len();

                instructions.push(Instruction::JumpIfNonZero(open_index));
                instructions[open_index] = Instruction::JumpIfZero(close_index);
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }

    if !bracket_stack.is_empty() {
        return Err(ParseError::UnmatchedOpenBracket);
    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instructions_are_collapsed_correctly() {
        let result = parse("+++---+>>><<<>").unwrap();
        assert_eq!(result, vec![Instruction::Add(1), Instruction::Move(1)]);
    }

    #[test]
    fn parse_parses_simple_commands() {
        let result = parse("+-><.,").unwrap();
        assert_eq!(result, vec![Instruction::Write, Instruction::Read,]);
    }

    #[test]
    fn parse_ignores_non_command_characters() {
        let result = parse("+ hello \n\t - world").unwrap();
        assert_eq!(result, vec![Instruction::Add(1), Instruction::Add(255)]);
    }

    #[test]
    fn parse_resolves_simple_loop() {
        let result = parse("[-]").unwrap();
        assert_eq!(
            result,
            vec![
                Instruction::JumpIfZero(2),
                Instruction::Add(255),
                Instruction::JumpIfNonZero(0),
            ]
        );
    }

    #[test]
    fn resolves_nested_loops() {
        let result = parse("[[-]]").unwrap();
        assert_eq!(
            result,
            vec![
                Instruction::JumpIfZero(4),
                Instruction::JumpIfZero(3),
                Instruction::Add(255),
                Instruction::JumpIfNonZero(1),
                Instruction::JumpIfNonZero(0),
            ]
        );
    }

    #[test]
    fn parse_errors_if_unmatched_open_bracket() {
        assert_eq!(parse("[-"), Err(ParseError::UnmatchedOpenBracket));
    }

    #[test]
    fn parse_errors_if_unmatched_close_bracket() {
        assert_eq!(parse("-]"), Err(ParseError::UnmatchedCloseBracket));
    }
}

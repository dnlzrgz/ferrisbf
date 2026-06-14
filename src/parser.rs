use crate::error::ParseError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    MoveRight,            // >
    MoveLeft,             // <
    Inc,                  // +
    Dec,                  // -
    Output,               // .
    Input,                // ,
    JumpIfZero(usize),    // [
    JumpIfNonZero(usize), // ]
}

/// Parses Brainfunc source code into a sequence of instructions.
pub fn parse(source: &str) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut bracket_stack: Vec<usize> = Vec::new();

    for ch in source.chars() {
        match ch {
            '>' => instructions.push(Instruction::MoveRight),
            '<' => instructions.push(Instruction::MoveLeft),
            '+' => instructions.push(Instruction::Inc),
            '-' => instructions.push(Instruction::Dec),
            '.' => instructions.push(Instruction::Output),
            ',' => instructions.push(Instruction::Input),
            '[' => {
                // target is known at this time, so we use 0.
                instructions.push(Instruction::JumpIfZero(0));
                bracket_stack.push(instructions.len() - 1);
            }
            ']' => {
                let open_index = bracket_stack
                    .pop()
                    .ok_or(ParseError::UnmatchedCloseBracket)?;
                let close_index = instructions.len();

                instructions.push(Instruction::JumpIfNonZero(open_index));
                instructions[open_index] = Instruction::JumpIfZero(close_index);
            }
            _ => {}
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
    fn parse_parses_simple_commands() {
        let result = parse("+-><.,").unwrap();
        assert_eq!(
            result,
            vec![
                Instruction::Inc,
                Instruction::Dec,
                Instruction::MoveRight,
                Instruction::MoveLeft,
                Instruction::Output,
                Instruction::Input,
            ]
        );
    }

    #[test]
    fn parse_ignores_non_command_characters() {
        let result = parse("+ hello \n\t - world").unwrap();
        assert_eq!(result, vec![Instruction::Inc, Instruction::Dec]);
    }

    #[test]
    fn parse_resolves_simple_loop() {
        let result = parse("[-]").unwrap();
        assert_eq!(
            result,
            vec![
                Instruction::JumpIfZero(2),
                Instruction::Dec,
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
                Instruction::Dec,
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

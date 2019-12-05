use ::std::convert::TryFrom;

fn main() -> std::io::Result<()> {
    let input = get_input()?;

    let mut program = IntCode::new(input.into());
    program.execute();

    println!("1: {}", program);

    Ok(())
}

#[derive(Debug)]
enum OpCode {
    Add {
        input_a: usize,
        immediate_a: bool,

        input_b: usize,
        immediate_b: bool,

        output: usize,
    },
    Multiply {
        input_a: usize,
        immediate_a: bool,

        input_b: usize,
        immediate_b: bool,

        output: usize,
    },
    Save {
        address: usize,
    },
    Load {
        address: usize,
    },
    Done,
}

impl TryFrom<Vec<i32>> for OpCode {
    type Error = &'static str;

    fn try_from(value: Vec<i32>) -> Result<Self, Self::Error> {
        match parse_instruction(value[0]) {
            (1, a, b) => Ok(OpCode::Add {
                input_a: value[1] as usize,
                immediate_a: a,
                input_b: value[2] as usize,
                immediate_b: b,
                output: value[3] as usize,
            }),
            (2, a, b) => Ok(OpCode::Multiply {
                input_a: value[1] as usize,
                immediate_a: a,
                input_b: value[2] as usize,
                immediate_b: b,
                output: value[3] as usize,
            }),
            (3, _, _) => Ok(OpCode::Save {
                address: value[1] as usize,
            }),
            (4, _, _) => Ok(OpCode::Load {
                address: value[1] as usize,
            }),
            (99, _, _) => Ok(OpCode::Done),
            _ => Err("no such code"),
        }
    }

}

fn parse_instruction(instruction: i32) -> (i32, bool, bool) {
    (instruction%100, instruction/100%10 == 1, instruction/1000%10 == 1)
}

#[derive(Debug)]
struct IntCode {
    data: Vec<i32>,
    cursor_position: usize,
    finished: bool,
}

impl std::fmt::Display for IntCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data[0])
    }
}

impl IntCode {
    fn new(data: Vec<i32>) -> IntCode {
        IntCode {
            data,
            cursor_position: 0,
            finished: false,
        }
    }

    fn execute(&mut self) {
        while self.parsable() {
            self.parse();
        }
    }

    fn parsable(&self) -> bool {
        !self.finished && self.cursor_position < self.data.len()
    }

    fn get_instructions(&mut self) -> Vec<i32> {
        let instruction: i32 = self.data[self.cursor_position] % 10;
        let mut result = self.data.as_slice();
        match instruction {
            1 | 2 => {
                result = &self.data[self.cursor_position..=self.cursor_position + 3];
                self.cursor_position = self.cursor_position + 4;
            }
            3 | 4 => {
                result = &self.data[self.cursor_position..=self.cursor_position + 1];
                self.cursor_position = self.cursor_position + 2;
            }
            9 => {
                result = &self.data[self.cursor_position..=self.cursor_position];
                self.cursor_position = self.cursor_position + 1;
            }
            _ => panic!("danger"),
        }

        result.into()
    }

    fn parse(&mut self) {
        let instruction = self.get_instructions();
        let op_code = OpCode::try_from(instruction).unwrap();
        match op_code {
            OpCode::Done => self.finished = true,
            OpCode::Add {
                input_a,
                immediate_a,
                input_b,
                immediate_b,
                output,
            } => {
                self.data[output] = if immediate_a {
                    input_a as i32
                } else {
                    self.data[input_a]
                } + if immediate_b {
                    input_b as i32
                } else {
                    self.data[input_b]
                }
            }
            OpCode::Multiply {
                input_a,
                immediate_a,
                input_b,
                immediate_b,
                output,
            } => {
                self.data[output] = if immediate_a {
                    input_a as i32
                } else {
                    self.data[input_a]
                } * if immediate_b {
                    input_b as i32
                } else {
                    self.data[input_b]
                }
            }
            OpCode::Save { address } => self.data[address] = 1,
            OpCode::Load { address } => println!("{}", self.data[address]),
        };
    }
}

fn get_input() -> std::io::Result<Vec<i32>> {
    let string = include_str!("./input.txt").trim();

    let data = string
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_execute() {
        let mut program = IntCode::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        program.execute();
        assert_eq!(program.data[0], 3500);
    }
}

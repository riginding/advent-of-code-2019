use ::std::convert::TryFrom;

fn main() -> std::io::Result<()> {
    let input = get_input()?;

    let mut program = IntCode::new(input.clone(), 1);
    program.execute();
    println!("1: {}", program.output);

    let mut program = IntCode::new(input.into(), 5);
    program.execute();
    println!("1: {}", program.output);
    Ok(())
}

#[derive(Debug)]
enum OpCode {
    Add {
        input_a: i64,
        immediate_a: bool,
        input_b: i64,
        immediate_b: bool,
        output: i64,
    },
    Multiply {
        input_a: i64,
        immediate_a: bool,
        input_b: i64,
        immediate_b: bool,
        output: i64,
    },
    Save {
        address: usize,
    },
    Load {
        immediate_load: bool,
        address: usize,
    },
    JumpIfTrue {
        comparison: usize,
        immediate_cmp: bool,
        adress: usize,
    },
    JumpIfFalse {
        comparison: usize,
        immediate_cmp: bool,
        adress: usize,
    },
    LessThan {
        comparison_a: usize,
        immediate_a: bool,
        comparison_b: usize,
        immediate_b: bool,
        result: usize,
    },
    Equals {
        comparison_a: usize,
        immediate_a: bool,
        comparison_b: usize,
        immediate_b: bool,
        result: usize,
    },
    Done,
}

impl TryFrom<Vec<i64>> for OpCode {
    type Error = &'static str;

    fn try_from(value: Vec<i64>) -> Result<Self, Self::Error> {
        match parse_instruction(value[0]) {
            (1, a, b) => Ok(OpCode::Add {
                input_a: value[1],
                immediate_a: a,
                input_b: value[2],
                immediate_b: b,
                output: value[3],
            }),
            (2, a, b) => Ok(OpCode::Multiply {
                input_a: value[1],
                immediate_a: a,
                input_b: value[2],
                immediate_b: b,
                output: value[3],
            }),
            (3, _, _) => Ok(OpCode::Save {
                address: value[1] as usize,
            }),
            (4, immediate_load, _) => Ok({
                OpCode::Load {
                    immediate_load,
                    address: value[1] as usize,
                }
            }),
            (5, cmp, _) => Ok(OpCode::JumpIfTrue {
                comparison: value[1] as usize,
                immediate_cmp: cmp,
                adress: value[2] as usize,
            }),
            (6, cmp, _) => Ok(OpCode::JumpIfFalse {
                comparison: value[1] as usize,
                immediate_cmp: cmp,
                adress: value[2] as usize,
            }),
            (7, a, b) => Ok(OpCode::LessThan {
                comparison_a: value[1] as usize,
                immediate_a: a,
                comparison_b: value[2] as usize,
                immediate_b: b,
                result: value[3] as usize,
            }),
            (8, a, b) => Ok(OpCode::Equals {
                comparison_a: value[1] as usize,
                immediate_a: a,
                comparison_b: value[2] as usize,
                immediate_b: b,
                result: value[3] as usize,
            }),
            (9, _, _) => Ok(OpCode::Done),
            _ => Err("no such code"),
        }
    }
}

fn parse_instruction(instruction: i64) -> (i64, bool, bool) {
    (
        instruction % 10,
        instruction / 100 % 10 == 1,
        instruction / 1000 % 10 == 1,
    )
}

#[derive(Debug)]
struct IntCode {
    data: Vec<i64>,
    cursor_position: usize,
    finished: bool,
    input: i64,
    output: i64,
}

impl std::fmt::Display for IntCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data[0])
    }
}

impl IntCode {
    fn new(data: Vec<i64>, input: i64) -> IntCode {
        IntCode {
            data,
            cursor_position: 0,
            finished: false,
            input,
            output: 0,
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

    fn get_instructions(&mut self) -> Vec<i64> {
        let instruction: i64 = self.data[self.cursor_position] % 10;
        let mut result = self.data.as_slice();
        match instruction {
            1 | 2 | 7 | 8 => {
                result = &self.data[self.cursor_position..=self.cursor_position + 3];
                self.cursor_position = self.cursor_position + 4;
            }
            5 | 6 => {
                result = &self.data[self.cursor_position..=self.cursor_position + 2];
                self.cursor_position = self.cursor_position + 3;
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
                let a = if immediate_a {
                    input_a as i64
                } else {
                    self.data[input_a as usize]
                };
                let b = if immediate_b {
                    input_b as i64
                } else {
                    self.data[input_b as usize]
                };

                dbg!(a);
                dbg!(b);
                self.data[output as usize] = a + b
            }
            OpCode::Multiply {
                input_a,
                immediate_a,
                input_b,
                immediate_b,
                output,
            } => {
                self.data[output as usize] = if immediate_a {
                    input_a as i64
                } else {
                    self.data[input_a as usize]
                } * if immediate_b {
                    input_b as i64
                } else {
                    self.data[input_b as usize]
                }
            }
            OpCode::Save { address } => self.data[address] = self.input,
            OpCode::Load {
                address,
                immediate_load,
            } => {
                if immediate_load {
                    self.output = address as i64;
                } else {
                    self.output = self.data[address];
                }
            }
            OpCode::JumpIfTrue {
                comparison,
                immediate_cmp,
                adress,
            } => {
                if immediate_cmp {
                    if comparison != 0 {
                        self.cursor_position = adress
                    }
                } else {
                    if self.data[comparison] != 0 {
                        self.cursor_position = adress
                    }
                }
            }
            OpCode::JumpIfFalse {
                comparison,
                immediate_cmp,
                adress,
            } => {
                if immediate_cmp {
                    if comparison == 0 {
                        self.cursor_position = adress
                    }
                } else {
                    if self.data[comparison] == 0 {
                        self.cursor_position = adress
                    }
                }
            }
            OpCode::LessThan {
                comparison_a,
                immediate_a,
                comparison_b,
                immediate_b,
                result,
            } => {
                if if { immediate_a } {
                    comparison_a
                } else {
                    self.data[comparison_a] as usize
                } < if { immediate_b } {
                    comparison_b
                } else {
                    self.data[comparison_b] as usize
                } {
                    self.data[result] = 1
                } else {
                    self.data[result] = 0
                }
            }
            OpCode::Equals {
                comparison_a,
                immediate_a,
                comparison_b,
                immediate_b,
                result,
            } => {
                if if { immediate_a } {
                    comparison_a
                } else {
                    self.data[comparison_a] as usize
                } == if { immediate_b } {
                    comparison_b
                } else {
                    self.data[comparison_b] as usize
                } {
                    self.data[result] = 1
                } else {
                    self.data[result] = 0
                }
            }
        };
    }
}

fn get_input() -> std::io::Result<Vec<i64>> {
    let string = include_str!("./input.txt").trim();

    let data = string
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_equal() {
        let mut program = IntCode::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8);
        program.execute();
        assert_eq!(program.output, 1);

        let mut program = IntCode::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 999);
        program.execute();
        assert_eq!(program.output, 0);
    }

    #[test]
    fn test_immediate_equal() {
        let mut program = IntCode::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8);
        program.execute();
        assert_eq!(program.output, 1);

        let mut program = IntCode::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 999);
        program.execute();
        assert_eq!(program.output, 0);
    }

    #[test]
    fn test_position_less_than() {
        let mut program = IntCode::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 1);
        program.execute();
        assert_eq!(program.output, 1);

        let mut program = IntCode::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 999);
        program.execute();
        assert_eq!(program.output, 0);
    }

    #[test]
    fn test_immediate_less_than() {
        let mut program = IntCode::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 1);
        program.execute();
        assert_eq!(program.output, 1);

        let mut program = IntCode::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 999);
        program.execute();
        assert_eq!(program.output, 0);
    }

    #[test]
    fn test_position_jump() {
        let mut program = IntCode::new(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            1,
        );
        program.execute();
        assert_eq!(program.output, 1);

        let mut program = IntCode::new(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            0,
        );
        program.execute();
        assert_eq!(program.output, 0);
    }

    #[test]
    fn test_immediate_jump() {
        let mut program = IntCode::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 1);
        program.execute();
        assert_eq!(program.output, 1);

        let mut program = IntCode::new(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0);
        program.execute();
        assert_eq!(program.output, 0);
    }

    #[test]
    fn test_large_example() {
        let mut program = IntCode::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            7,
        );
        program.execute();
        assert_eq!(program.output, 999);

        let mut program = IntCode::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            8,
        );
        program.execute();
        assert_eq!(program.output, 1000);

        let mut program = IntCode::new(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            9,
        );
        program.execute();
        assert_eq!(program.output, 1001);
    }
}

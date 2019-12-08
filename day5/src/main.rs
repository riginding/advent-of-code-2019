use ::std::convert::TryFrom;

fn main() -> std::io::Result<()> {
    let input = get_input()?;

    let mut program = IntCode::new(input.clone(), 1);
    program.execute();
    println!("1: {}", program.output);

    let mut program = IntCode::new(input.into(), 5);
    program.execute();
    println!("2: {}", program.output);

    Ok(())
}

#[derive(Debug)]
enum OpCode {
    Add {
        input_a: i32,
        immediate_a: bool,
        input_b: i32,
        immediate_b: bool,
        output: usize,
    },
    Multiply {
        input_a: i32,
        immediate_a: bool,
        input_b: i32,
        immediate_b: bool,
        output: usize,
    },
    Save {
        address: usize,
    },
    Load {
        immediate_load: bool,
        address: usize,
    },
    JumpIfTrue {
        comparison: i32,
        immediate_cmp: bool,
        address: usize,
        immediate_adr: bool,
    },
    JumpIfFalse {
        comparison: i32,
        immediate_cmp: bool,
        address: usize,
        immediate_adr: bool,
    },
    LessThan {
        comparison_a: i32,
        immediate_a: bool,
        comparison_b: i32,
        immediate_b: bool,
        result: usize,
    },
    Equals {
        comparison_a: i32,
        immediate_a: bool,
        comparison_b: i32,
        immediate_b: bool,
        result: usize,
    },
    Done,
}

impl TryFrom<Vec<i32>> for OpCode {
    type Error = &'static str;

    fn try_from(value: Vec<i32>) -> Result<Self, Self::Error> {
        match parse_instruction(value[0]) {
            (1, a, b) => Ok(OpCode::Add {
                input_a: value[1],
                immediate_a: a,
                input_b: value[2],
                immediate_b: b,
                output: value[3] as usize,
            }),
            (2, a, b) => Ok(OpCode::Multiply {
                input_a: value[1],
                immediate_a: a,
                input_b: value[2],
                immediate_b: b,
                output: value[3] as usize,
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
            (5, cmp, adr) => Ok(OpCode::JumpIfTrue {
                comparison: value[1],
                immediate_cmp: cmp,
                address: value[2] as usize,
                immediate_adr: adr,
            }),
            (6, cmp, adr) => Ok(OpCode::JumpIfFalse {
                comparison: value[1],
                immediate_cmp: cmp,
                address: value[2] as usize,
                immediate_adr: adr,
            }),
            (7, a, b) => Ok(OpCode::LessThan {
                comparison_a: value[1],
                immediate_a: a,
                comparison_b: value[2],
                immediate_b: b,
                result: value[3] as usize,
            }),
            (8, a, b) => Ok(OpCode::Equals {
                comparison_a: value[1],
                immediate_a: a,
                comparison_b: value[2],
                immediate_b: b,
                result: value[3] as usize,
            }),
            (9, _, _) => Ok(OpCode::Done),
            _ => Err("no such code"),
        }
    }
}

fn parse_instruction(instruction: i32) -> (i32, bool, bool) {
    (
        instruction % 10,
        instruction / 100 % 10 == 1,
        instruction / 1_000 % 10 == 1,
    )
}

#[derive(Debug)]
struct IntCode {
    data: Vec<i32>,
    cursor_position: usize,
    finished: bool,
    input: i32,
    output: i32,
}

impl std::fmt::Display for IntCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data[0])
    }
}

impl IntCode {
    fn new(data: Vec<i32>, input: i32) -> IntCode {
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

    fn get_instructions(&mut self) -> Vec<i32> {
        let instruction: i32 = self.data[self.cursor_position] % 10;
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
            }
            _ => panic!("danger"),
        }

        result.into()
    }

    fn parse(&mut self) {
        let instruction = self.get_instructions();
        let op_code = OpCode::try_from(instruction).unwrap();
        match op_code {
            OpCode::Done => {
                self.finished = true;
            }
            OpCode::Add {
                input_a,
                immediate_a,
                input_b,
                immediate_b,
                output,
            } => {
                let a = if immediate_a {
                    input_a
                } else {
                    self.data[input_a as usize]
                };
                let b = if immediate_b {
                    input_b
                } else {
                    self.data[input_b as usize]
                };

                self.data[output as usize] = a + b;
            }
            OpCode::Multiply {
                input_a,
                immediate_a,
                input_b,
                immediate_b,
                output,
            } => {
                let a = if immediate_a {
                    input_a
                } else {
                    self.data[input_a as usize]
                };
                let b = if immediate_b {
                    input_b
                } else {
                    self.data[input_b as usize]
                };
                self.data[output as usize] = a * b
            }
            OpCode::Save { address } => {
                self.data[address] = self.input;
            }
            OpCode::Load {
                address,
                immediate_load,
            } => {
                if immediate_load {
                    self.output = address as i32;
                } else {
                    self.output = self.data[address];
                }
            }
            OpCode::JumpIfTrue {
                comparison,
                immediate_cmp,
                address,
                immediate_adr,
            } => {
                if immediate_cmp {
                    if comparison != 0 {
                        if immediate_adr {
                            self.cursor_position = address;
                        } else {
                            self.cursor_position = self.data[address] as usize;
                        }
                    }
                } else {
                    if self.data[comparison as usize] != 0 {
                        if immediate_adr {
                            self.cursor_position = address;
                        } else {
                            self.cursor_position = self.data[address] as usize;
                        }
                    }
                }
            }
            OpCode::JumpIfFalse {
                comparison,
                immediate_cmp,
                address,
                immediate_adr,
            } => {
                if immediate_cmp {
                    if comparison == 0 {
                        if immediate_adr {
                            self.cursor_position = address;
                        } else {
                            self.cursor_position = self.data[address] as usize;
                        }
                    }
                } else {
                    if self.data[comparison as usize] == 0 {
                        if immediate_adr {
                            self.cursor_position = address;
                        } else {
                            self.cursor_position = self.data[address] as usize;
                        }
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
                let a = if { immediate_a } {
                    comparison_a
                } else {
                    self.data[comparison_a as usize]
                };
                let b = if { immediate_b } {
                    comparison_b
                } else {
                    self.data[comparison_b as usize]
                };
                if a < b {
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
                let a = if { immediate_a } {
                    comparison_a
                } else {
                    self.data[comparison_a as usize]
                };

                let b = if { immediate_b } {
                    comparison_b
                } else {
                    self.data[comparison_b as usize]
                };

                if a == b {
                    self.data[result] = 1
                } else {
                    self.data[result] = 0
                }
            }
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
    fn countdown() {
        let mut program = IntCode::new(vec![101, -1, 7, 7, 4, 7, 1105, 11, 0, 99], 1);
        program.execute();
        assert_eq!(program.output, 0);
    }

    #[test]
    fn extra_tests() {
        let mut program = IntCode::new(vec![1, 0, 3, 3, 1005, 2, 10, 5, 1, 0, 4, 1, 99], 1);
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

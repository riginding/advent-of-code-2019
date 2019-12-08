use ::std::convert::TryFrom;
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let input = get_input()?;
    
    let mut results = Vec::new();

    for c in (0..=4).permutations(5) {
        let output = get_max_thruster(&c, input.clone());
        results.push(output);
    };

    println!("1: {:?}", results.iter().max().unwrap());

    Ok(())
}

fn get_max_thruster(c: &[usize], input: Vec<i32>) -> i32 {
    let mut amp_a = IntCode::new(input.clone(), vec![0, c[0] as i32]);
    amp_a.execute();
    let output = amp_a.output.pop().expect("should have value");

    let mut amp_b = IntCode::new(input.clone(), vec![output, c[1] as i32]);
    amp_b.execute();
    let output = amp_b.output.pop().expect("should have value 1");

    let mut amp_c = IntCode::new(input.clone(), vec![output, c[2] as i32]);
    amp_c.execute();
    let output = amp_c.output.pop().expect("shoul have value 2");
    
    let mut amp_d = IntCode::new(input.clone(), vec![output, c[3] as i32]);
    amp_d.execute();
    let output = amp_d.output.pop().expect("should have value 3");

    let mut amp_e = IntCode::new(input.clone(), vec![output, c[4] as i32]);
    amp_e.execute();
    let output = amp_e.output.pop().expect("should have value4");

    output
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
    input: Vec<i32>,
    output: Vec<i32>,
}

impl std::fmt::Display for IntCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data[0])
    }
}

impl IntCode {
    fn new(data: Vec<i32>, input: Vec<i32>) -> IntCode {
        IntCode {
            data,
            cursor_position: 0,
            finished: false,
            input,
            output: vec![],
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
                self.data[address] = self.input.pop().unwrap();
            }
            OpCode::Load {
                address,
                immediate_load,
            } => {
                if immediate_load {
                    self.output.push(address as i32);
                } else {
                    self.output.push(self.data[address]);
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
    fn test_max_thruster() { 
      let output = get_max_thruster(&[4,3,2,1,0], vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]);
      assert_eq!(output, 43210);

        let output = get_max_thruster(&[0,1,2,3,4], vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
        101,5,23,23,1,24,23,23,4,23,99,0,0]);
        assert_eq!(output, 54321);

        let output = get_max_thruster(&[1,0,4,3,2], vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]);
        assert_eq!(output, 65210);
    } 
}

use ::std::convert::TryFrom;

fn main() -> std::io::Result<()> {
    let mut input = get_input()?;

    Ok(())
}

#[derive(Debug)]
enum OpCode {
    Add {
        input_a: usize,
        input_b: usize,
        output: usize,
    },
    Multiply {
        input_a: usize,
        input_b: usize,
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
        match value[0] {
            1 => Ok(OpCode::Add {
                input_a: value[1] as usize,
                input_b: value[2] as usize,
                output: value[3] as usize,
            }),
            2 => Ok(OpCode::Multiply {
                input_a: value[1] as usize,
                input_b: value[2] as usize,
                output: value[3] as usize,
            }),
            99 => Ok(OpCode::Done),
            _ => Err("no such code"),
        }
    }
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
        let result = &self.data[self.cursor_position..=self.cursor_position + 3];
        self.cursor_position = self.cursor_position + 4;

        result.into()
    }

    fn parse(&mut self) {
        let instruction = self.get_instructions();
        let op_code = OpCode::try_from(instruction).unwrap();
        match op_code {
            OpCode::Done => self.finished = true,
            OpCode::Add {
                input_a,
                input_b,
                output,
            } => self.data[output] = self.data[input_a] + self.data[input_b],
            OpCode::Multiply {
                input_a,
                input_b,
                output,
            } => self.data[output] = self.data[input_a] * self.data[input_b],
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

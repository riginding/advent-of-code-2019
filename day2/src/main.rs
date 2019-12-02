use ::std::convert::TryFrom;

fn main() -> std::io::Result<()> {
    let mut input = get_input()?;

    println!("1: {}", part1(&mut input));
    part2(&mut input);

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
        write!(f, "result: {}", self.data[0])
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
            } => {
                self.data[output] = self.data[input_a] + self.data[input_b]
            }
            OpCode::Multiply {
                input_a,
                input_b,
                output,
            } => {
                self.data[output] = self.data[input_a] * self.data[input_b]
            }
        };
    }
}

fn get_input() -> std::io::Result<Vec<i32>> {
    let string = include_str!("./input").trim();

    let data = string
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Ok(data)
}

fn part1(input: &mut [i32]) -> IntCode {
    let mut program = IntCode::new(input.into());
    program.execute();

    program
}

// TODO make this nicer :)
fn part2(input: &[i32]) {
    let mut all: Vec<(i32, i32)> = Vec::new();

    let input_one: Vec<i32> = (0..=99).map(|n| n ).collect();
    let input_two: Vec<i32> = (0..=99).map(|n| n ).collect();

    for i in &input_one {
        for j in &input_two {
            all.push((*i, *j));
        }
    }

    let mut found = false;
    while !found {
        let possible_input = all.pop().unwrap();
        let mut data: Vec<i32> = input.into();
        data[1] = possible_input.0;
        data[2] = possible_input.1;
        let mut program = IntCode::new(data.into());
        program.execute();

        found = program.data[0] == 19690720;
        if(found) {
            print!("{}", 100 * possible_input.0 + possible_input.1)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // replace this with real test
    #[test]
    fn test_this() {
        assert!(true)
    }
}

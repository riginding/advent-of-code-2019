use ::std::convert::TryFrom;

fn main() -> std::io::Result<()> {
    let mut input = grab_input()?;

    part1(&mut input);
    part2(&mut input);
    Ok(())
}

enum OpCode {
    Add,
    Multiply,
    Done,
}

impl TryFrom<i32> for OpCode {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Multiply),
            99 => Ok(OpCode::Done),
            _ => Err("no such code"),
        }
    }
}

fn grab_input() -> std::io::Result<Vec<i32>> {
    let string = include_str!("./input").trim();

    let data = string
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Ok(data)
}

fn part1(input: &mut [i32]) -> Vec<i32> {
    unimplemented!();
}

fn part2(input: &[i32]) -> i32 {
    unimplemented!();
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

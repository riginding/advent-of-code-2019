fn main() {
    get_instructions();
}

#[derive(Debug)]
enum Direction {
    Down(i32),
    Up(i32),
    Left(i32),
    Right(i32),
}

type Directions = Vec<Direction>;

fn get_instructions() -> Vec<Directions> {
    let strings = include_str!("./input.txt");
    let lines: Vec<&str> = strings.lines().collect();
    let directions: Vec<Directions> = lines.iter().map(|x| {
        let data: Vec<&str> = x.split(',').collect();
        dbg!(&data);
        data
    }
    ).collect();


    unimplemented!();
}

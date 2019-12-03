use ::std::convert::TryFrom;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let (first, second) = get_wires();
    part1(first, second);

    let (first, second) = get_wires();
    part2(first, second);
}

fn part1(first: Directions, second: Directions) {
    let mut origin = Point { x: 0, y: 0 };
    let first_path_points: HashSet<Point> = HashSet::from_iter(origin.draw_wires(first));
    let second_path_points: HashSet<Point> = HashSet::from_iter(origin.draw_wires(second));

    let intersection_of_paths = first_path_points.intersection(&second_path_points);

    let mut min_value = 999_999;

    for point in intersection_of_paths {
        let current_value = point.manhattan_distance();
        if current_value < min_value {
            min_value = current_value;
        }
    }

    println!("1: {}", min_value);
}

fn part2(first: Directions, second: Directions) {
    unimplemented!();
}

#[derive(Debug)]
enum Direction {
    Down(i32),
    Up(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn draw_wires(&mut self, wire: Directions) -> Vec<Point> {
        let mut start = Point {
            x: self.x,
            y: self.y,
        };
        let mut points = Vec::new();

        for direction in wire {
            let (new_points, new_start) = start.draw(direction);
            start = new_start;
            points.extend_from_slice(&new_points);
        }

        points
    }

    fn draw(&self, direction: Direction) -> (Vec<Point>, Point) {
        let start = Point {
            x: self.x,
            y: self.y,
        };
        let points = match direction {
            Direction::Up(times) => start.up(times),
            Direction::Down(times) => start.down(times),
            Direction::Left(times) => start.left(times),
            Direction::Right(times) => start.right(times),
        };

        let last_point = points.last().unwrap().clone();
        (points, last_point)
    }

    fn up(&self, times: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for i in 0..times {
            points.push(Point {
                x: self.x,
                y: self.y + i + 1,
            });
        }

        points
    }

    fn down(&self, times: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for i in 0..times {
            points.push(Point {
                x: self.x,
                y: self.y - i - 1,
            });
        }

        points
    }

    fn right(&self, times: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for i in 0..times {
            points.push(Point {
                x: self.x + i + 1,
                y: self.y,
            });
        }

        points
    }

    fn left(&self, times: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for i in 0..times {
            points.push(Point {
                x: self.x - i - 1,
                y: self.y,
            });
        }

        points
    }
}

impl TryFrom<&str> for Direction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.chars().next() {
            Some('D') => Ok(Direction::Down(value[1..].parse::<i32>().unwrap())),
            Some('U') => Ok(Direction::Up(value[1..].parse::<i32>().unwrap())),
            Some('L') => Ok(Direction::Left(value[1..].parse::<i32>().unwrap())),
            Some('R') => Ok(Direction::Right(value[1..].parse::<i32>().unwrap())),
            _ => Err("no such direction"),
        }
    }
}

type Directions = Vec<Direction>;

fn get_wires() -> (Directions, Directions) {
    let strings = include_str!("./input.txt");
    let lines: Vec<&str> = strings.lines().collect();
    let first_path: Directions = lines[0]
        .split(",")
        .map(|x| Direction::try_from(x).unwrap())
        .collect();
    let second_path: Directions = lines[1]
        .split(",")
        .map(|x| Direction::try_from(x).unwrap())
        .collect();

    (first_path, second_path)
}

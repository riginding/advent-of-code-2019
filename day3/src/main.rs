use ::std::convert::TryFrom;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

fn main() {
    let (first, second) = get_wires();
    part1(first, second);

    let (mut first, mut second) = get_wires();
    part2(first, second);
}

fn part1(first: Directions, second: Directions) {
    let mut origin = Point {
        x: 0,
        y: 0,
        step_1: None,
        step_2: None,
    };
    let first_path_points: HashSet<Point> = HashSet::from_iter(origin.draw_wires(first, true));
    let second_path_points: HashSet<Point> = HashSet::from_iter(origin.draw_wires(second, false));

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

fn part2(mut first: Directions, mut second: Directions) {
    let mut origin = Point {
        x: 0,
        y: 0,
        step_1: None,
        step_2: None,
    };
    let first_path_points: HashSet<Point> = HashSet::from_iter(origin.draw_wires(first, true));
    let second_path_points: HashSet<Point> = HashSet::from_iter(origin.draw_wires(second, false));

    let intersection_of_paths = first_path_points.intersection(&second_path_points);
    dbg!(intersection_of_paths);
}

#[derive(Debug)]
enum Direction {
    Down(i32),
    Up(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
    step_1: Option<i32>,
    step_2: Option<i32>,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Point {
    fn new1(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
            step_1: Some(0),
            step_2: None,
        }
    }

    fn new2(x: i32, y: i32) -> Point {
        Point {
            x,
            y,
            step_1: None,
            step_2: Some(0),
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn draw_wires(&mut self, wire: Directions, first: bool) -> Vec<Point> {
        let mut start = Point {
            x: self.x,
            y: self.y,
            step_1: self.step_1,
            step_2: self.step_2,
        };
        let mut points = Vec::new();

        for direction in wire {
            let (new_points, new_start) = start.draw(direction, first);
            start = new_start;
            points.extend_from_slice(&new_points);
        }

        points
    }

    fn draw(&self, direction: Direction, first: bool) -> (Vec<Point>, Point) {
        let start = Point {
            x: self.x,
            y: self.y,
            step_1: self.step_1,
            step_2: self.step_2,
        };
        let points = match direction {
            Direction::Up(times) => start.up(times, first),
            Direction::Down(times) => start.down(times, first),
            Direction::Left(times) => start.left(times, first),
            Direction::Right(times) => start.right(times, first),
        };

        let last_point = points.last().unwrap().clone();
        (points, last_point)
    }

    fn up(&self, times: i32, first: bool) -> Vec<Point> {
        let mut points = Vec::new();

        for i in 0..times {
            points.push(if first {
                Point {
                    x: self.x,
                    y: self.y + i + 1,
                    step_1: Some(match self.step_1 {
                        Some(x) => x + 1,
                        None => 1,
                    }),
                    step_2: self.step_2,
                }
            } else {
                Point {
                    x: self.x,
                    y: self.y + i + 1,
                    step_1: self.step_1,
                    step_2: Some(match self.step_2 {
                        Some(x) => x + 1,
                        None => 1,
                    }),
                }
            })
        }

        points
    }

    fn down(&self, times: i32, first: bool) -> Vec<Point> {
        let mut points = Vec::new();

        for i in 0..times {
            points.push(if first {
                Point {
                    x: self.x,
                    y: self.y - i - 1,
                    step_1: Some(match self.step_1 {
                        Some(x) => x + 1,
                        None => 1,
                    }),
                    step_2: self.step_2,
                }
            } else {
                Point {
                    x: self.x,
                    y: self.y - i - 1,
                    step_1: self.step_1,
                    step_2: Some(match self.step_2 {
                        Some(x) => x + 1,
                        None => 1,
                    }),
                }
            })
        }

        points
    }

    fn right(&self, times: i32, first: bool) -> Vec<Point> {
        let mut points = Vec::new();

        for i in 0..times {
            points.push(if first {
                Point {
                    x: self.x + i + 1,
                    y: self.y,
                    step_1: Some(match self.step_1 {
                        Some(x) => x + 1,
                        None => 1,
                    }),
                    step_2: self.step_2,
                }
            } else {
                Point {
                    x: self.x + i + 1,
                    y: self.y,
                    step_1: self.step_1,
                    step_2: Some(match self.step_2 {
                        Some(x) => x + 1,
                        None => 1,
                    }),
                }
            });
        }

        points
    }

    fn left(&self, times: i32, first: bool) -> Vec<Point> {
        let mut points = Vec::new();

        for i in 0..times {
            points.push(if first {
                Point {
                    x: self.x - i - 1,
                    y: self.y,
                    step_1: Some(match self.step_1 {
                        Some(x) => x + 1,
                        None => 1,
                    }),
                    step_2: self.step_2,
                }
            } else {
                Point {
                    x: self.x - i - 1,
                    y: self.y,
                    step_1: self.step_1,
                    step_2: Some(match self.step_2 {
                        Some(x) => x + 1,
                        None => 1,
                    }),
                }
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

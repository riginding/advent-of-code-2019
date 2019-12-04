use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    println!("1: {}", find_intersection(input, CostFunction::Manhattan));
    println!("2: {}", find_intersection(input, CostFunction::Steps));
}

enum CostFunction {
    Manhattan,
    Steps,
}

fn find_intersection(input: &str, cost: CostFunction) -> i32 {
    // (Point): (first|second): step_count
    let mut map: HashMap<(i32, i32), HashMap<&str, i32>> = Default::default();
    let mut orig_wires = vec!["first", "second"];
    let mut wires = orig_wires.clone();

    for line in input.lines() {
        let mut x = 0i32;
        let mut y = 0i32;
        let wire = wires.pop().unwrap();

        let mut step_counter = 0;
        for instruction in line.split(',') {
            let (step_x, step_y) = match instruction.chars().next().unwrap() {
                'L' => (-1, 0),
                'R' => (1, 0),
                'D' => (0, -1),
                'U' => (0, 1),
                _ => panic!("Error while parsing instruction"),
            };

            for _ in 0..instruction[1..].parse::<i32>().unwrap() {
                x += step_x;
                y += step_y;

                step_counter += 1;
                *map.entry((x, y)).or_default().entry(wire).or_default() = step_counter;
            }
        }
    }

    map.into_iter()
        .filter(|(_, matches)| matches.len() == orig_wires.len())
        .map(|((x, y), steps)| match cost {
            CostFunction::Manhattan => x.abs() + y.abs(),
            CostFunction::Steps => steps.values().sum::<i32>(),
        })
        .min()
        .unwrap_or(0)
}

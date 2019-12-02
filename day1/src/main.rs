use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let input = grab_input()?;

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));

    Ok(())
}

fn part1(input: &[i32]) -> i32 {
    input.iter().fold(0, |mut acc, num| {
        acc = acc + calculate_fuel(*num);
        acc
    })
}

fn part2(input: &[i32]) -> i32 {
    input.iter().fold(0, |mut acc, num| {
        acc = acc + calculate_recursive_fuel(*num);
        acc
    })
}

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate_recursive_fuel(mass: i32) -> i32 {
    let _mass = calculate_fuel(mass);

    if _mass > 0 {
        _mass + calculate_recursive_fuel(_mass)
    } else {
        0
    }
}

fn grab_input() -> std::io::Result<Vec<i32>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let input: Vec<i32> = reader
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse().unwrap())
        .collect();

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_calculate_recursive_fuel() {
        assert_eq!(calculate_recursive_fuel(1969), 966);
        assert_eq!(calculate_recursive_fuel(100756), 50346);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[12]), 2);
        assert_eq!(part1(&[12, 12, 12]), 6);
        assert_eq!(part1(&[120, 399, 42]), 181);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&[12]), 2);
        assert_eq!(part2(&[12, 12, 12]), 6);
        assert_eq!(part2(&[120, 399, 42]), 247);
    }
}

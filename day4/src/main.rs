fn main() {
    part1();
    part2();
}

fn part1() {
    let mut results = Vec::new();
    for i in 245318..=765747 {
        let nums = number_to_vec(i);
        if has_six_digit(&nums) && has_increasing_numbers(&nums) && has_double(&nums) {
            results.push(i);
        }
    }

    println!("1: {}", results.len());
}

fn part2() {
    let mut results = Vec::new();
    for i in 245318..=765747 {
        let nums = number_to_vec(i);
        if has_six_digit(&nums) && has_increasing_numbers(&nums) && has_only_double(&nums) {
            results.push(i);
        }
    }

    println!("1: {}", results.len());
}

fn has_six_digit(val: &[i32]) -> bool {
    val.len() == 6
}

fn has_double(val: &[i32]) -> bool {
    (val[0] == val[1])
        || (val[1] == val[2])
        || (val[2] == val[3])
        || (val[3] == val[4])
        || (val[4] == val[5])
}

fn has_only_double(val: &[i32]) -> bool {
    (val[0] == val[1] && val[1] != val[2])
        || (val[1] == val[2] && val[1] != val[0] && val[2] != val[3])
        || (val[2] == val[3] && val[2] != val[1] && val[3] != val[4])
        || (val[3] == val[4] && val[3] != val[2] && val[4] != val[5])
        || (val[4] == val[5] && val[4] != val[3])
}

fn has_increasing_numbers(val: &[i32]) -> bool {
    (val[0] <= val[1])
        && (val[1] <= val[2])
        && (val[2] <= val[3])
        && (val[3] <= val[4])
        && (val[4] <= val[5])
}

fn number_to_vec(n: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing() {
        assert_eq!(has_increasing_numbers(&number_to_vec(245322)), false);
        assert_eq!(has_increasing_numbers(&number_to_vec(123456)), true);
        assert_eq!(has_increasing_numbers(&number_to_vec(111111)), true);
    }

    #[test]
    fn test_only_double() {
        assert_eq!(has_only_double(&number_to_vec(222222)), false);
        assert_eq!(has_only_double(&number_to_vec(112233)), true);
        assert_eq!(has_only_double(&number_to_vec(111122)), true);
    }
}

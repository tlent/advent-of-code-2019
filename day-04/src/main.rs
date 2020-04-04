const RANGE_START: u32 = 123257;
const RANGE_END: u32 = 647015;

fn main() {
    println!("Part one: {}", solve_part_one());
    println!("Part two: {}", solve_part_two());
}

fn solve_part_one() -> u32 {
    (RANGE_START..=RANGE_END)
        .filter(|&val| has_matching_adjacent_digits(val))
        .filter(|&val| has_nondecreasing_digits(val))
        .count() as u32
}

fn solve_part_two() -> u32 {
    (RANGE_START..=RANGE_END)
        .filter(|&val| has_nondecreasing_digits(val))
        .filter(|&val| has_two_matching_adacent_digits(val))
        .count() as u32
}

fn has_matching_adjacent_digits(mut value: u32) -> bool {
    let mut prev = value % 10;
    value /= 10;
    while value > 0 {
        let current = value % 10;
        if current == prev {
            return true;
        }
        prev = current;
        value /= 10;
    }
    false
}

fn has_two_matching_adacent_digits(mut value: u32) -> bool {
    let mut count = 1;
    let mut digit = value % 10;
    value /= 10;
    while value > 0 {
        let current = value % 10;
        if count == 2 && current != digit {
            return true;
        }
        if current == digit {
            count += 1;
        } else {
            count = 1;
            digit = current;
        }
        value /= 10;
    }
    count == 2
}

fn has_nondecreasing_digits(mut value: u32) -> bool {
    let mut prev_digit = value % 10;
    value /= 10;
    while value > 0 {
        if value % 10 > prev_digit {
            return false;
        }
        prev_digit = value % 10;
        value /= 10;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    const PART_ONE_SOLUTION: u32 = 2220;
    const PART_TWO_SOLUTION: u32 = 1515;

    #[test]
    fn part_one() {
        assert_eq!(solve_part_one(), PART_ONE_SOLUTION);
    }

    #[test]
    fn part_two() {
        assert_eq!(solve_part_two(), PART_TWO_SOLUTION);
    }

    #[test]
    fn test_adjacent_digits() {
        assert!(has_matching_adjacent_digits(112345));
        assert!(!has_matching_adjacent_digits(123456));
    }

    #[test]
    fn test_nondecreasing_digits() {
        assert!(has_nondecreasing_digits(112233));
        assert!(!has_nondecreasing_digits(112211));
    }

    #[test]
    fn test_two_adjacent_digits() {
        assert!(has_two_matching_adacent_digits(112345));
        assert!(has_two_matching_adacent_digits(234511));
        assert!(!has_two_matching_adacent_digits(111222));
    }
}

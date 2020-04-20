const INPUT: &str = include_str!("../input.txt");

fn main() {
    let numbers = parse_input();
    println!("Part one: {}", solve_part_one(&numbers));
    println!("Part two: {}", solve_part_two(&numbers));
}

fn parse_input() -> Vec<u8> {
    INPUT.trim().bytes().map(|b| b - b'0').collect()
}

fn solve_part_one(numbers: &[u8]) -> String {
    let mut numbers = numbers.to_vec();
    for _ in 0..100 {
        numbers = phase(&numbers);
    }
    numbers[0..8].iter().map(ToString::to_string).collect()
}

fn phase(numbers: &[u8]) -> Vec<u8> {
    let numbers: Vec<_> = numbers.iter().map(|&v| v as i32).collect();
    (0..numbers.len())
        .map(|i| {
            let chunks = numbers[i..].chunks(i + 1);
            let sum = chunks.clone().step_by(4).flatten().sum::<i32>()
                - chunks.skip(2).step_by(4).flatten().sum::<i32>();
            (sum.abs() % 10) as u8
        })
        .collect()
}

fn solve_part_two(numbers: &[u8]) -> String {
    let offset = message_offset(numbers);
    let length = 10000 * numbers.len();
    // for this method to work this must be true
    assert!(offset > length / 2);

    let mut numbers: Vec<_> = numbers
        .iter()
        .cycle()
        .take(length)
        .skip(offset)
        .copied()
        .collect();
    for _ in 0..100 {
        numbers = part_two_phase(&numbers);
    }
    numbers[0..8].iter().map(ToString::to_string).collect()
}

fn part_two_phase(numbers: &[u8]) -> Vec<u8> {
    let mut digits = Vec::with_capacity(numbers.len());
    let mut digit = 0;
    for &val in numbers.iter().rev() {
        digit = (digit + val) % 10;
        digits.push(digit);
    }
    digits.reverse();
    digits
}

fn message_offset(numbers: &[u8]) -> usize {
    let mut offset = 0;
    for &x in &numbers[0..7] {
        offset *= 10;
        offset += x as usize;
    }
    offset
}

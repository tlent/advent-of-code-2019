const INPUT: &str = include_str!("../input.txt");

type Mass = u32;
type FuelRequirement = u32;

fn main() {
    let masses = parse_input();
    println!("Part one: {}", solve_part_one(&masses));
    println!("Part two: {}", solve_part_two(&masses));
}

fn parse_input() -> Vec<Mass> {
    INPUT.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve_part_one(masses: &[Mass]) -> FuelRequirement {
    masses.iter().copied().map(basic_fuel_requirement).sum()
}

fn solve_part_two(masses: &[Mass]) -> FuelRequirement {
    masses.iter().copied().map(full_fuel_requirement).sum()
}

fn basic_fuel_requirement(mass: Mass) -> FuelRequirement {
    (mass / 3).checked_sub(2).unwrap_or(0)
}

fn full_fuel_requirement(mass: Mass) -> FuelRequirement {
    let mut total = 0;
    let mut req = basic_fuel_requirement(mass);
    while req > 0 {
        total += req;
        req = basic_fuel_requirement(req);
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;
    const PART_ONE_SOLUTION: FuelRequirement = 3_397_667;
    const PART_TWO_SOLUTION: FuelRequirement = 5_093_620;

    #[test]
    fn part_one() {
        let masses = parse_input();
        assert_eq!(solve_part_one(&masses), PART_ONE_SOLUTION);
    }

    #[test]
    fn part_two() {
        let masses = parse_input();
        assert_eq!(solve_part_two(&masses), PART_TWO_SOLUTION);
    }
}

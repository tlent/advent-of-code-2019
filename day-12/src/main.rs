use std::cmp::Ordering;
use std::ops::{Add, AddAssign};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let moons = Moon::moons_from_input(INPUT);
    println!("{}", solve_part_one(&moons));
    println!("{}", solve_part_two(&moons));
}

fn solve_part_one(moons: &[Moon]) -> i32 {
    let mut moons = moons.to_vec();
    for _ in 0..1000 {
        step(&mut moons);
    }
    moons.into_iter().map(|moon| moon.energy()).sum()
}

fn step(moons: &mut [Moon]) {
    let positions: Vec<_> = moons.iter().map(|moon| moon.position).collect();
    for moon in moons.iter_mut() {
        moon.apply_gravity(&positions);
    }
    for moon in moons.iter_mut() {
        moon.apply_velocity();
    }
}

fn solve_part_two(initial_state: &[Moon]) -> usize {
    let mut state = initial_state.to_vec();
    let mut steps = 0;
    let mut x_loop = None;
    let mut y_loop = None;
    let mut z_loop = None;
    while x_loop.is_none() || y_loop.is_none() || z_loop.is_none() {
        step(&mut state);
        steps += 1;

        let x_matches_initial = matches(initial_state, &state, |moon| {
            (moon.position.x, moon.velocity.x)
        });
        if x_loop.is_none() && x_matches_initial {
            x_loop = Some(steps);
        }

        let y_matches_initial = matches(initial_state, &state, |moon| {
            (moon.position.y, moon.velocity.y)
        });
        if y_loop.is_none() && y_matches_initial {
            y_loop = Some(steps);
        }

        let z_matches_initial = matches(initial_state, &state, |moon| {
            (moon.position.z, moon.velocity.z)
        });
        if z_loop.is_none() && z_matches_initial {
            z_loop = Some(steps);
        }
    }
    let x_loop = x_loop.unwrap();
    let y_loop = y_loop.unwrap();
    let z_loop = z_loop.unwrap();
    least_common_multiple(&[x_loop, y_loop, z_loop])
}

fn matches<F, T, U>(a: &[T], b: &[T], selector: F) -> bool
where
    F: FnMut(&T) -> U + Copy,
    U: Eq,
{
    let a_values = a.iter().map(selector);
    let b_values = b.iter().map(selector);
    a_values.zip(b_values).all(|(a, b)| a == b)
}

fn least_common_multiple(values: &[usize]) -> usize {
    let mut a = values[0];
    for &b in values {
        a = a * b / greatest_common_divisor(a, b);
    }
    a
}

fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Moon {
    position: Vec3,
    velocity: Vec3,
}

impl Moon {
    fn moons_from_input(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let content = &line[1..line.len() - 1];
                let mut values = content.split(',').map(|s| {
                    let s = s.trim();
                    let s = &s[2..];
                    s.parse().unwrap()
                });
                let x = values.next().unwrap();
                let y = values.next().unwrap();
                let z = values.next().unwrap();
                Self {
                    position: Vec3 { x, y, z },
                    velocity: Default::default(),
                }
            })
            .collect()
    }

    fn apply_gravity(&mut self, others_positions: &[Vec3]) {
        for other in others_positions {
            self.velocity.x += match other.x.cmp(&self.position.x) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            };
            self.velocity.y += match other.y.cmp(&self.position.y) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            };
            self.velocity.z += match other.z.cmp(&self.position.z) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            };
        }
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    fn energy(&self) -> i32 {
        let Vec3 { x, y, z } = self.position;
        let potential: i32 = [x, y, z].iter().map(|v| v.abs()).sum();
        let Vec3 { x, y, z } = self.velocity;
        let kinetic: i32 = [x, y, z].iter().map(|v| v.abs()).sum();
        potential * kinetic
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let orbits = parse_input();
    println!("Part one: {}", solve_part_one(&orbits));
    println!("Part two: {}", solve_part_two(&orbits));
}

fn parse_input() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in INPUT.lines() {
        let mut parts = line.split(')');
        let object = String::from(parts.next().unwrap());
        let orbiter = String::from(parts.next().unwrap());
        map.insert(orbiter, object);
    }
    map
}

fn solve_part_one(orbits: &HashMap<String, String>) -> u32 {
    let mut orbit_count = 0;
    for object in orbits.keys() {
        let mut o = object;
        while o != "COM" {
            orbit_count += 1;
            o = &orbits[o];
        }
    }
    orbit_count
}

fn solve_part_two(orbits: &HashMap<String, String>) -> u32 {
    let you_to_com = path_to_com(orbits, "YOU");
    let san_to_com = path_to_com(orbits, "SAN");
    let mut path_intersection = None;
    let mut steps = 0;
    for object in you_to_com {
        steps += 1;
        if san_to_com.contains(&object) {
            path_intersection = Some(object);
            break;
        }
    }
    let path_intersection = path_intersection.expect("No intersection found");
    for object in san_to_com {
        steps += 1;
        if object == path_intersection {
            break;
        }
    }
    steps
}

fn path_to_com(orbits: &HashMap<String, String>, object: &str) -> Vec<String> {
    let mut path = vec![];
    let mut orbit = &orbits[object];
    while orbit != "COM" {
        orbit = &orbits[orbit];
        path.push(orbit.clone());
    }
    path
}

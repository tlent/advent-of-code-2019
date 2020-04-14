use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let reactions = parse_input(INPUT);
    println!("{}", solve_part_one(&reactions));
    println!("{}", solve_part_two(&reactions));
}

fn solve_part_one(reactions: &HashMap<String, Reaction>) -> usize {
    ore_required(reactions, 1)
}

fn solve_part_two(reactions: &HashMap<String, Reaction>) -> usize {
    let ore = 1_000_000_000_000;
    let ore_per_fuel = ore_required(reactions, 1);
    let mut start = ore / ore_per_fuel;
    let mut end = 2 * start;
    while start <= end {
        let mid = (start + end) / 2;
        if ore_required(reactions, mid) > ore {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
    end
}

#[derive(Debug)]
struct Reaction {
    inputs: Vec<ReactionComponent>,
    output: ReactionComponent,
}

#[derive(Debug)]
struct ReactionComponent {
    count: usize,
    chemical: String,
}

fn parse_input(input: &str) -> HashMap<String, Reaction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split("=>");

            let left = parts.next().unwrap();
            let inputs = left
                .split(',')
                .map(|s| {
                    let mut parts = s.trim().split(' ');
                    let count = parts.next().map(|s| s.parse().unwrap()).unwrap();
                    let chemical = String::from(parts.next().unwrap());
                    ReactionComponent { count, chemical }
                })
                .collect();

            let right = parts.next().unwrap().trim();
            let mut parts = right.split(' ');
            let count = parts.next().map(|s| s.parse().unwrap()).unwrap();
            let chemical = String::from(parts.next().unwrap());
            let output = ReactionComponent {
                count,
                chemical: chemical.clone(),
            };
            (chemical, Reaction { inputs, output })
        })
        .collect()
}

fn ore_required(reactions: &HashMap<String, Reaction>, fuel: usize) -> usize {
    let blocks = find_blocks(&reactions);
    let mut ore = 0;
    let mut required = HashMap::new();
    required.insert("FUEL", fuel);
    while !required.is_empty() {
        let chemicals: Vec<_> = required.keys().copied().collect();
        let mut blocked: HashSet<&str> = HashSet::new();
        for &chemical in &chemicals {
            blocked.extend(&blocks[chemical]);
        }
        for &chemical in &chemicals {
            if blocked.contains(&chemical) {
                continue;
            }
            let count = required.remove(chemical).unwrap();
            let reaction = &reactions[chemical];
            let multiplier = count / reaction.output.count
                + if count % reaction.output.count == 0 {
                    0
                } else {
                    1
                };
            for component in &reaction.inputs {
                let count = component.count * multiplier;
                if component.chemical == "ORE" {
                    ore += count;
                    continue;
                }
                *required.entry(&component.chemical).or_insert(0) += count;
            }
        }
    }
    ore
}

fn find_blocks(reactions: &HashMap<String, Reaction>) -> HashMap<&str, HashSet<&str>> {
    let mut result = HashMap::new();
    for (chemical, reaction) in reactions {
        let mut blocks = HashSet::new();
        let mut stack: Vec<_> = reaction
            .inputs
            .iter()
            .map(|r| r.chemical.as_str())
            .collect();
        while let Some(chemical) = stack.pop() {
            if chemical == "ORE" {
                continue;
            }
            blocks.insert(chemical);
            let r = &reactions[chemical];
            for ReactionComponent { chemical, .. } in &r.inputs {
                if chemical == "ORE" {
                    continue;
                }
                stack.push(chemical.as_str());
            }
        }
        result.insert(chemical.as_str(), blocks);
    }
    result
}

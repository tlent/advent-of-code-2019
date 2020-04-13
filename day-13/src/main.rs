const INPUT: &str = include_str!("../input.txt");

mod intcode;

use intcode::{Int, Program};
use std::collections::HashMap;
use std::{cmp::Ordering, fmt};

type Tiles = HashMap<(usize, usize), Tile>;

fn main() {
    let program = Program::from_input(INPUT);
    println!("{}", solve_part_one(&program));
    solve_part_two(&program);
}

fn solve_part_one(program: &Program) -> usize {
    let mut program = program.clone();
    let output = program.run(&[]);
    let mut tiles = HashMap::new();
    parse_output(&mut tiles, &output);
    tiles.values().filter(|&&tile| tile == Tile::Block).count()
}

fn parse_output(tiles: &mut Tiles, output: &[Int]) {
    for tile_info in output.chunks(3) {
        let x = tile_info[0];
        let y = tile_info[1];
        let id = tile_info[2];
        if x == -1 && y == 0 {
            println!("Score: {}", id);
            continue;
        }
        let x = x as usize;
        let y = y as usize;
        let tile = Tile::from_intcode(id);
        tiles.insert((x, y), tile);
    }
}

fn solve_part_two(program: &Program) {
    let mut program = program.clone();
    program.write_to_memory(0, 2);
    let mut tiles = HashMap::new();
    let mut input = 0;
    loop {
        let output = program.run(&[input]);
        parse_output(&mut tiles, &output);
        let (ball_x, _) = find_tile(&tiles, Tile::Ball);
        let (paddle_x, _) = find_tile(&tiles, Tile::Paddle);
        input = match ball_x.cmp(&paddle_x) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        if !tiles.values().any(|&tile| tile == Tile::Block) {
            break;
        }
    }
}

fn find_tile(tiles: &Tiles, target: Tile) -> (usize, usize) {
    tiles
        .iter()
        .find_map(
            |(&pos, &tile)| {
                if tile == target {
                    Some(pos)
                } else {
                    None
                }
            },
        )
        .unwrap()
}

fn print_tiles(tiles: &Tiles) {
    let max_x = tiles.keys().map(|&(x, _)| x).max().unwrap();
    let max_y = tiles.keys().map(|&(_, y)| y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", tiles[&(x, y)]);
        }
        println!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn from_intcode(intcode: Int) -> Self {
        match intcode {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            _ => panic!("invalid intcode"),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Empty => ' ',
            Self::Wall => '#',
            Self::Block => '*',
            Self::Paddle => '=',
            Self::Ball => 'o',
        };
        write!(f, "{}", c)
    }
}

use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    time::Instant,
};

use helpers::{print_day, print_solution, read_input};

mod helpers;

#[derive(Debug)]
enum Square {
    EmptySpace,
    Mirror(Direction),
    Splitter(Direction),
}

impl FromStr for Square {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Square::EmptySpace),
            "/" => Ok(Square::Mirror(Direction::Up)),
            "\\" => Ok(Square::Mirror(Direction::Down)),
            "-" => Ok(Square::Splitter(Direction::Right)),
            "|" => Ok(Square::Splitter(Direction::Up)),
            _ => Err(()),
        }
    }

    type Err = ();
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (usize, usize);
type Beam = (Position, Direction);

pub fn part_one(input: &str) -> Option<String> {
    let (grid, max_y, max_x) = get_grid(input);
    let (start_pos, start_dir) = ((0, 0), Direction::Right);

    get_energized(start_pos, start_dir, &grid, max_y, max_x)
        .to_string()
        .into()
}

fn get_grid(input: &str) -> (HashMap<(usize, usize), Square>, usize, usize) {
    let s = input.trim().split('\n').collect::<Vec<&str>>();
    let mut grid: HashMap<Position, Square> = HashMap::new();
    let max_y = s.len() - 1;
    let max_x = s[0].len() - 1;

    for (y, row) in s.iter().enumerate() {
        for (x, col) in row.chars().enumerate() {
            let square = Square::from_str(&col.to_string()).unwrap();
            grid.insert((x, y), square);
        }
    }
    (grid, max_y, max_x)
}

fn get_energized(
    start_pos: Position,
    start_dir: Direction,
    grid: &HashMap<(usize, usize), Square>,
    max_y: usize,
    max_x: usize,
) -> usize {
    let mut beams: Vec<Beam> = vec![(start_pos, start_dir)];
    let mut energized: HashSet<Position> = HashSet::new();
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();

    while let Some((position, direction)) = beams.pop() {
        if visited.contains(&(position.0, position.1, direction.clone())) {
            continue;
        }
        if let Some(square) = grid.get(&position) {
            energized.insert(position);
            visited.insert((position.0, position.1, direction.clone()));
            match &square {
                Square::EmptySpace => {
                    add_move(position, direction.clone(), max_y, max_x, &mut beams);
                }
                Square::Mirror(mirror_direction) => match &direction {
                    Direction::Up => match mirror_direction {
                        Direction::Up => {
                            add_move(position, Direction::Right, max_y, max_x, &mut beams);
                        }
                        Direction::Down => {
                            add_move(position, Direction::Left, max_y, max_x, &mut beams);
                        }
                        _ => panic!("Invalid mirror direction"),
                    },
                    Direction::Down => match mirror_direction {
                        Direction::Up => {
                            add_move(position, Direction::Left, max_y, max_x, &mut beams);
                        }
                        Direction::Down => {
                            add_move(position, Direction::Right, max_y, max_x, &mut beams);
                        }
                        _ => panic!("Invalid mirror direction"),
                    },
                    Direction::Left => match mirror_direction {
                        Direction::Up => {
                            add_move(position, Direction::Down, max_y, max_x, &mut beams);
                        }
                        Direction::Down => {
                            add_move(position, Direction::Up, max_y, max_x, &mut beams);
                        }
                        _ => panic!("Invalid mirror direction"),
                    },
                    Direction::Right => match mirror_direction {
                        Direction::Up => {
                            add_move(position, Direction::Up, max_y, max_x, &mut beams);
                        }
                        Direction::Down => {
                            add_move(position, Direction::Down, max_y, max_x, &mut beams);
                        }
                        _ => panic!("Invalid mirror direction"),
                    },
                },

                Square::Splitter(splitter_dir) => match &direction {
                    Direction::Up => match splitter_dir {
                        Direction::Up => {
                            add_move(position, direction.clone(), max_y, max_x, &mut beams);
                        }
                        Direction::Right => {
                            add_move(position, Direction::Left, max_y, max_x, &mut beams);
                            add_move(position, Direction::Right, max_y, max_x, &mut beams);
                        }
                        _ => panic!("Invalid splitter direction"),
                    },
                    Direction::Down => match splitter_dir {
                        Direction::Up => {
                            add_move(position, direction.clone(), max_y, max_x, &mut beams);
                        }
                        Direction::Right => {
                            add_move(position, Direction::Left, max_y, max_x, &mut beams);
                            add_move(position, Direction::Right, max_y, max_x, &mut beams);
                        }
                        _ => panic!("Invalid splitter direction"),
                    },
                    Direction::Left => match splitter_dir {
                        Direction::Up => {
                            add_move(position, Direction::Up, max_y, max_x, &mut beams);
                            add_move(position, Direction::Down, max_y, max_x, &mut beams);
                        }
                        Direction::Right => {
                            add_move(position, direction.clone(), max_y, max_x, &mut beams);
                        }
                        _ => panic!("Invalid splitter direction"),
                    },
                    Direction::Right => match splitter_dir {
                        Direction::Up => {
                            add_move(position, Direction::Up, max_y, max_x, &mut beams);
                            add_move(position, Direction::Down, max_y, max_x, &mut beams);
                        }
                        Direction::Right => {
                            add_move(position, direction.clone(), max_y, max_x, &mut beams);
                        }
                        _ => panic!("Invalid splitter direction"),
                    },
                },
            };
        }
    }

    energized.len()
}

fn add_move(
    position: (usize, usize),
    d: Direction,
    max_y: usize,
    max_x: usize,
    beams: &mut Vec<Beam>,
) {
    if let Some(new_position) = move_beam(position, &d, max_y, max_x) {
        beams.push((new_position, d));
    }
}

fn move_beam((x, y): Position, dir: &Direction, max_y: usize, max_x: usize) -> Option<Position> {
    match dir {
        Direction::Up => {
            if y == 0 {
                None
            } else {
                Some((x, y - 1))
            }
        }
        Direction::Down => {
            if y == max_y {
                None
            } else {
                Some((x, y + 1))
            }
        }
        Direction::Left => {
            if x == 0 {
                None
            } else {
                Some((x - 1, y))
            }
        }
        Direction::Right => {
            if x == max_x {
                None
            } else {
                Some((x + 1, y))
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let (grid, max_y, max_x) = get_grid(input);

    let timer = Instant::now();

    let mut par: Vec<(Position, Direction)> = vec![];

    for x in 0..max_x {
        par.push(((x, 0), Direction::Down));
        par.push(((x, max_y), Direction::Down));
    }

    for y in 0..max_y {
        par.push(((0, y), Direction::Right));
        par.push(((max_x, y), Direction::Left));
    }

    let maxx = par
        .par_iter()
        .map(|(pos, dir)| get_energized(*pos, dir.clone(), &grid, max_y, max_x))
        .max();

    let ret = maxx.unwrap().to_string().into();
    let elapsed = timer.elapsed();

    println!("Timer: {:.2?}", elapsed);
    ret
}

fn main() {
    let input = read_input();
    print_day();
    print_solution(1, part_one, &input);
    print_solution(2, part_two, &input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::read_example;

    #[test]
    fn test_part_one() {
        let input = read_example();
        assert_eq!(part_one(&input), Some("46".to_string()));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input), None);
    }
}

use std::str::FromStr;

use helpers::{print_day, print_solution, read_input};
use itertools::Itertools;
use rayon::prelude::*;

mod helpers;

#[derive(Debug)]
enum Cube {
    Blue(u32),
    Red(u32),
    Green(u32),
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num_str, color) = s.trim().split_once(' ').unwrap();
        let num = num_str.parse::<u32>().unwrap();

        match color {
            "blue" => Ok(Cube::Blue(num)),
            "red" => Ok(Cube::Red(num)),
            "green" => Ok(Cube::Green(num)),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let games = input
        .trim()
        .split('\n')
        .enumerate()
        .map(|(i, l)| {
            let game_id = i + 1;
            let (red, green, blue) = (12, 13, 14);
            let (_, game) = l.split_once(':').unwrap();
            let mut possible = true;
            for pull in game.split(';') {
                for a in pull.split(',') {
                    let cube = a.parse::<Cube>().unwrap();

                    match cube {
                        Cube::Blue(n) => {
                            if n > blue {
                                possible = false;
                            }
                        }
                        Cube::Red(n) => {
                            if n > red {
                                possible = false;
                            }
                        }
                        Cube::Green(n) => {
                            if n > green {
                                possible = false;
                            }
                        }
                    }
                }
            }
            if possible {
                game_id
            } else {
                0
            }
        })
        .collect_vec();

    games.iter().sum::<usize>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .trim()
        .split('\n')
        .map(|l| {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            let (_, game) = l.split_once(':').unwrap();
            for pull in game.split(';') {
                for a in pull.split(',') {
                    let cube = a.parse::<Cube>().unwrap();

                    match cube {
                        Cube::Blue(n) => {
                            if n > blue {
                                blue = n;
                            }
                        }
                        Cube::Red(n) => {
                            if n > red {
                                red = n;
                            }
                        }
                        Cube::Green(n) => {
                            if n > green {
                                green = n;
                            }
                        }
                    }
                }
            }
            red * green * blue
        })
        .sum::<u32>()
        .into()
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
    #[ignore]
    fn test_part_one() {
        let input = read_example();
        assert_eq!(part_one(&input), 32.into());
    }

    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input), 2286.into());
    }
}

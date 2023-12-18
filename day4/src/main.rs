use helpers::{print_day, print_solution, read_input};
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

mod helpers;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (_, numbers) = l.split_once(": ").unwrap();

            let (winning, ours) = numbers.split_once('|').unwrap();

            let winning: HashSet<_> = winning
                .split_whitespace()
                .filter_map(|s| {
                    if s == " " {
                        None
                    } else {
                        Some(u32::from_str(s).unwrap())
                    }
                })
                .collect();

            let wins = ours
                .split_whitespace()
                .filter_map(|s| {
                    if s == " " {
                        None
                    } else {
                        let num = u32::from_str(s).unwrap();
                        if winning.contains(&num) {
                            Some(num)
                        } else {
                            None
                        }
                    }
                })
                .count();

            match wins {
                0 => 0,
                x => 2_u32.pow(x as u32 - 1),
            }
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: HashMap<usize, u32> = HashMap::new();
    let lines: Vec<&str> = input.split('\n').filter(|l| !l.is_empty()).collect();
    lines.iter().enumerate().for_each(|(i, l)| {
        let card_number = i + 1;
        let copies = cards.get(&card_number).unwrap_or(&0) + 1;
        let (_, numbers) = l.split_once(": ").unwrap();

        let (winning, ours) = numbers.split_once('|').unwrap();

        let winning: HashSet<_> = winning
            .split_whitespace()
            .filter_map(|s| {
                if s == " " {
                    None
                } else {
                    Some(u32::from_str(s).unwrap())
                }
            })
            .collect();

        let wins = ours
            .split_whitespace()
            .filter_map(|s| {
                if s == " " {
                    None
                } else {
                    let num = u32::from_str(s).unwrap();
                    if winning.contains(&num) {
                        Some(num)
                    } else {
                        None
                    }
                }
            })
            .count();

        for i in 1..wins + 1 {
            let won_card = card_number + i;
            cards.insert(won_card, cards.get(&won_card).unwrap_or(&0) + copies);
        }
    });

    (lines.len() as u32 + cards.values().sum::<u32>()).into()
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
        assert_eq!(part_one(&input), Some(13));
    }
    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input), Some(30));
    }
}

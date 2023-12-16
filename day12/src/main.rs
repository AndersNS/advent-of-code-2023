use helpers::{print_day, print_solution, read_input};
use std::str::FromStr;

mod helpers;

pub fn part_two(input: &str) -> Option<u32> {
    let massaged = input.split('\n').fold(String::new(), |mut acc, l| {
        let (s, n) = l.split_once(' ').unwrap();
        acc.push_str(&format!("{s}?{s}?{s}?{s}?{s} {n},{n},{n},{n},{n}\n"));
        acc
    });
    part_one(&massaged)
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split('\n')
        .map(|s| {
            let (str, groups) = parse_line(s);
            combinations(str, groups)
        })
        .sum::<u32>()
        .into()
}

fn parse_line(str: &str) -> (String, Vec<usize>) {
    let (first, second) = str.split_once(' ').unwrap();

    let groups = second
        .split(',')
        .map(usize::from_str)
        .map(|a| a.unwrap())
        .collect::<Vec<_>>();

    (first.to_string(), groups)
}

fn combinations(str: String, groups: Vec<usize>) -> u32 {
    permutations(&str)
        .iter()
        .filter(|p| combination_matches(p, &groups))
        .count() as u32
}

fn permutations(start_string: &str) -> Vec<String> {
    let mut queue: Vec<String> = vec![start_string.to_string()];
    let mut perms: Vec<String> = vec![];

    while let Some(str) = queue.pop() {
        if str.find('?').is_some() {
            let with_spring = str.replacen('?', "#", 1);
            let without_spring = str.replacen('?', ".", 1);
            queue.push(with_spring);
            queue.push(without_spring);
        } else {
            perms.push(str);
        }
    }

    perms
}

fn combination_matches(str: &str, groups: &[usize]) -> bool {
    let springs = str.split('.').filter(|a| !a.is_empty()).collect::<Vec<_>>(); // Only springs

    if springs.len() != groups.len() {
        // If we have more spring groups than desired
        return false;
    }

    springs
        .iter()
        .zip(groups.iter()) // Join springs with group sizes
        .all(|(spring, instr)| spring.len() == *instr)
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
    fn generate_params() {
        let perms = permutations("?");
        assert_eq!(perms, vec!["#", "."]);
    }

    #[test]
    fn check_string_valid() {
        let res = combination_matches("..#.", &[1]);
        assert!(res);
    }

    #[test]
    fn check_string_vaid() {
        let res = combination_matches("#", &[1]);
        assert!(res);
    }

    #[test]
    fn check_string_invalid() {
        let res = combination_matches("..#.", &[2]);
        assert!(!res);
    }

    #[test]
    fn perms() {
        let res = combinations("??".to_string(), vec![1]);
        assert_eq!(res, 2);
    }

    #[test]
    fn gen_perms() {
        let res = permutations("??");
        assert_eq!(res.len(), 4);
    }
    #[test]
    fn test_part_one() {
        let input = read_example();
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = read_example();
        assert_eq!(part_two(&input), None);
    }
}

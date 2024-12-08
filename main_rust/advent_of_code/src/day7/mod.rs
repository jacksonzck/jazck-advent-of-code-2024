use std::usize;

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
enum Operators {
    ADDITION,
    MULTIPLY
}

impl Operators {
    fn apply(self, first: usize, next: usize) -> usize {
        match self {
            Self::ADDITION => first + next, 
            Self::MULTIPLY => first * next,
        }
    }
}

#[derive(EnumIter, Debug, PartialEq)]
enum Cooler_Operators {
    ADDITION,
    MULTIPLY,
    CONCATENATE
}

impl Cooler_Operators {
    fn apply(self, first: usize, next: usize) -> usize {
        match self {
            Self::ADDITION => first + next, 
            Self::MULTIPLY => first * next,
            Self::CONCATENATE => (first.to_string() + &next.to_string()).parse().unwrap()
        }
    }
}


fn part1solution(input: &str) -> usize {
    input.split_terminator('\n').map(|a| a.split(":").collect_vec()).map(|a| (a[0].parse::<usize>().unwrap(), a[1].split_whitespace().map(|b| b.parse::<usize>().unwrap()).collect_vec())).
    map(|(get_to, items)| {
        let mut results = vec![items[0]];
        for item in items[1..].into_iter() {
            let mut next_results = vec![];
            for result in results.clone() {
                next_results.extend(Operators::iter().map(|operator| operator.apply(result, *item)));
            }
            results = next_results;
        }
        (get_to, results)
    }).filter(|(get_to, items) | items.iter().any(|item| item == get_to)).map(|(a, _)| a).sum()
}

fn part2solution(input: &str) -> usize {
    input.split_terminator('\n').map(|a| a.split(":").collect_vec()).map(|a| (a[0].parse::<usize>().unwrap(), a[1].split_whitespace().map(|b| b.parse::<usize>().unwrap()).collect_vec())).
    map(|(get_to, items)| {
        let mut results = vec![items[0]];
        for item in items[1..].into_iter() {
            let mut next_results = vec![];
            for result in results.clone() {
                next_results.extend(Cooler_Operators::iter().map(|operator| operator.apply(result, *item)));
            }
            results = next_results;
        }
        (get_to, results)
    }).filter(|(get_to, items) | items.iter().any(|item| item == get_to)).map(|(a, _)| a).sum()
}

#[cfg(test)]
mod tests {
    use crate::day7::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("day7input.example")), 3749);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("day7input.real")));
        assert_eq!(part1solution(include_str!("day7input.real")), 28730327770375);
    }

    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("day7input.example")), 11387);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("day7input.real")));
        assert_eq!(part2solution(include_str!("day7input.real")), 424977609625985);
    }
}
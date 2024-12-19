use itertools::Itertools;
use memoize::memoize;
use regex::Regex;

fn part1solution(input: &str) -> usize {
    let input: Vec<&str> = input.split("\n\n").collect();
    let towel_string = input[0];
    let towel_designs_string = input[1];
    let towels_regex = Regex::new(("(?:(".to_owned() + &towel_string.split(", ").join(")|(") + "))+$").as_str()).unwrap();
    towel_designs_string.split_whitespace().filter(|towel_design| towels_regex.is_match(&towel_design)).count()
}



fn part2solution(input: &str) -> usize {
    let input: Vec<&str> = input.split("\n\n").collect();
    let towel_string = input[0];
    let towels: Vec<String> = towel_string.split(", ").map(|f| f.to_owned()).collect();
    let towel_designs_string = input[1];
    // If only we had overlapping regex D:
    #[memoize]
    fn exploration(explore: String, towels: Vec<String>) -> usize {
        let mut cool_count = 0;
        for matchee in &towels {
            if explore.starts_with(matchee) {
                if explore.len() == matchee.len() {
                    cool_count += 1;
                } else {
                    cool_count += exploration(explore[matchee.len()..].to_owned(), towels.clone())
                }
            }
        }
        cool_count
    }
    towel_designs_string.split_whitespace().map(|towel_design| exploration(towel_design.to_string(), towels.clone())).sum()
}

#[cfg(test)]
mod tests {
    use crate::day19::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("input.example")), 6);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("input.real")));
        assert_eq!(part1solution(include_str!("input.real")), 94444);
    }
    
    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("input.example")), 16);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("input.real")));
        assert_eq!(part2solution(include_str!("input.real")), 636483903099279);
    }
}
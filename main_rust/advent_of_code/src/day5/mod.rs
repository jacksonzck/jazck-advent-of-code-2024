use std::collections::{HashMap, HashSet};

use regex::Regex;
fn part1solution(input: &str) -> i32 {
    let rule_regex = Regex::new(r"\d+\|\d+").unwrap();
    let rules = rule_regex.captures_iter(input).map(|f| f.get(0).unwrap().as_str());
    let update_regex = Regex::new(r"(?:\d+,)+\d+").unwrap();
    let updates= update_regex.captures_iter(input).map(|f| f.get(0).unwrap().as_str());
    let mut rule_map = HashMap::new();
    for rule in rules {
        let rule: Vec<_> = rule.split("|").collect();
        let before = rule[0];
        let later = rule[1];
        match rule_map.get_mut(later) {
            None => {rule_map.insert(later, vec![before]);},
            Some(before_vec) => before_vec.push(before),
        }
    }
    let mut good_updates = vec![];
    let default = vec!["f"];
    for update in updates {
        let mut bad_pages = HashSet::new();
        let mut is_good_map = true;
        for page in update.split(",").map(|f| f) {
            if bad_pages.contains(&page) {
                is_good_map = false;
                break
            }
            for broken_rule in rule_map.get(&page).unwrap_or(&default) {
                bad_pages.insert(broken_rule);
            }
        }
        if is_good_map {
            good_updates.push(update);
        }
    }
    let mut sum_middle = 0;
    for update in good_updates {
        let split_updates = update.split(",");
        sum_middle += split_updates.clone().skip(split_updates.count() / 2).next().unwrap().parse::<i32>().unwrap();
    }
    sum_middle
}

fn part2solution(input: &str) -> i32 {
    let rule_regex = Regex::new(r"\d+\|\d+").unwrap();
    let rules = rule_regex.captures_iter(input).map(|f| f.get(0).unwrap().as_str());
    let update_regex = Regex::new(r"(?:\d+,)+\d+").unwrap();
    let updates= update_regex.captures_iter(input).map(|f| f.get(0).unwrap().as_str());
    let mut rule_map = HashMap::new();
    for rule in rules {
        let rule: Vec<_> = rule.split("|").collect();
        let before = rule[0];
        let later = rule[1];
        match rule_map.get_mut(later) {
            None => {rule_map.insert(later, vec![before]);},
            Some(before_vec) => before_vec.push(before),
        }
    }
    //let mut good_updates = vec![];
    fn update_sorter<'a>(update: Vec<&'a str>, rule_map: HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
        let default = vec!["f"];
        let update_len = update.len();
        let mut new_updates = update.clone();
        let mut is_sorted = false;
        while !is_sorted {
            let update = new_updates.clone();
            new_updates = vec![];
            is_sorted = true;
            let mut bad_pages = HashSet::new();
            let mut new_to_sort = vec![];
            for (page_id, page) in update.clone().into_iter().enumerate() {
                if bad_pages.contains(&page) {
                    new_to_sort.push(page);
                    new_to_sort.append(new_updates.as_mut());
                    for i in page_id + 1..update_len {
                        new_to_sort.push(update[i])
                    }
                    is_sorted = false;
                    break
                }
                new_updates.push(page);
                for broken_rule in rule_map.get(&page).unwrap_or(&default) {
                    bad_pages.insert(broken_rule);
                }
            }
            if !is_sorted {
                new_updates = new_to_sort.clone()
            }
        }
        new_updates
    }
    let mut fixed_updates = vec![];
    for update in updates {
        let update: Vec<_> = update.split(",").collect();
        let sorted_update = update_sorter(update.clone(), rule_map.clone());
        if update == sorted_update {
            // It was a good update after all
            continue
        }
        fixed_updates.push(sorted_update);
    }
    let mut sum_middle = 0;
    for update in fixed_updates {
        //println!("{:#?}", update);
        sum_middle += update.clone().into_iter().skip(update.into_iter().count() / 2).next().unwrap().parse::<i32>().unwrap();
    }
    sum_middle
}

#[cfg(test)]
mod tests {
    use crate::day5::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("day5input.example")), 143);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("day5input.real")));
        assert_eq!(part1solution(include_str!("day5input.real")), 4689);
    }

    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("day5input.example")), 123);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("day5input.real")));
        assert_eq!(part2solution(include_str!("day5input.real")), 6336);
    }
}

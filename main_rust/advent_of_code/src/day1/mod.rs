fn part1solution(input: &str) -> i32 {
    let mut left_list: Vec<i32>  = vec![];
    let mut right_list: Vec<i32> = vec![];
    let mut left = true;
    for thing in input.split_whitespace() {
        match left {
            true => left_list.push(thing.parse().unwrap()),
            false => right_list.push(thing.parse().unwrap()),
        }
        left = !left;
    }
    left_list.sort();
    right_list.sort();
    let mut total_distance = 0;
    for (left_item, right_item) in left_list.into_iter().zip(right_list.into_iter()) {
        total_distance += (left_item - right_item).abs();
    }
    return total_distance;
}

fn part2solution(input: &str) -> i32 {
    let mut left_list: Vec<i32>  = vec![];
    let mut right_list: Vec<i32> = vec![];
    let mut left = true;
    for thing in input.split_whitespace() {
        match left {
            true => left_list.push(thing.parse().unwrap()),
            false => right_list.push(thing.parse().unwrap()),
        }
        left = !left;
    }
    let mut similarity = 0;
    for i in left_list {
        let count: i32 = i32::try_from(right_list.iter().filter(|item| **item == i).count()).unwrap();
        similarity += i * count
    }
    similarity
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1solution, part2solution};

    #[test]
    fn part1example() {
        let example_input = "3   4 
                         4   3
                         2   5
                         1   3
                         3   9
                         3   3";
        assert_eq!(part1solution(example_input), 11);
    }

    #[test]
    fn part1real() {
        let real_input = include_str!("day1input.txt");
        assert_eq!(part1solution(real_input), 2742123);
    }

    #[test]
    fn part2example() {
        let example_input = "3   4 
                         4   3
                         2   5
                         1   3
                         3   9
                         3   3";
        assert_eq!(part2solution(example_input), 31);
    }

    #[test]
    fn part2real() {
        let real_input = include_str!("day1input.txt");
        println!("{}", part2solution(real_input));
        assert_eq!(part2solution(real_input), 21328497);
    }
}
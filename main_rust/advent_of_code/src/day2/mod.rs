fn part1solution(input: &str) -> i32 {
    let mut levels = vec![];
    for level_input in input.split("\n") {
        let mut level: Vec<i32> = vec![];
        for number in level_input.split_whitespace() {
            level.push(number.parse().unwrap());
        }
        levels.push(level);
    }
    let mut num_safe_levels = 0;
    for mut og_level in levels.into_iter().filter(|level| !level.is_empty()) {
        //println!("{:#?}", og_level);
        let mut level = og_level.clone();
        let mut safe = true;
        let mut first = level.pop().unwrap();
        let mut last = level.pop().unwrap();
        let increasing = first > last;
        if (first - last).abs() > 3 || first - last == 0 {
            safe = false;
        }
        while !level.is_empty() {
            let mut new = level.pop().unwrap();
            if increasing && last < new {
                safe = false;
            } else if !increasing && last > new {
                safe = false;
            }
            if (last - new).abs() > 3 || last - new == 0 {
                safe = false;
            }
            last = new;
        }
        if safe {
            num_safe_levels += 1;
        }
    }
    num_safe_levels
}

fn part2solution(input: &str) -> i32 {
    let mut levels = vec![];
    for level_input in input.split("\n") {
        let mut level: Vec<i32> = vec![];
        for number in level_input.split_whitespace() {
            level.push(number.parse().unwrap());
        }
        levels.push(level);
    }
    let mut num_safe_levels = 0;
    fn is_safe_sublevel(og_level: Vec<i32>) -> bool {
        let mut level = og_level.clone();
        let mut safe = true;
        let mut first = level.pop().unwrap();
        let mut last = level.pop().unwrap();
        let increasing = first > last;
        if (first - last).abs() > 3 || first - last == 0 {
            safe = false;
        }
        while !level.is_empty() {
            let mut new = level.pop().unwrap();
            if increasing && last < new {
                safe = false;
            } else if !increasing && last > new {
                safe = false;
            }
            if (last - new).abs() > 3 || last - new == 0 {
                safe = false;
            }
            last = new;
        }
        safe
    }
    for mut og_level in levels.into_iter().filter(|level| !level.is_empty()) {
        let mut is_safe = false;
        for index in 0..og_level.len() {
            let mut sublevel = og_level.clone();
            sublevel.remove(index);
            if is_safe_sublevel(sublevel) {
                is_safe = true;
            }
        }
        if is_safe {
            num_safe_levels += 1;
        }
    }
    num_safe_levels
}


#[cfg(test)]
mod tests {
    use crate::day2::{part1solution, part2solution};


    #[test]
    fn part1example() {
        let example_input = include_str!("day2exampleinput.txt");
        assert_eq!(part1solution(example_input), 2);
    }

    #[test]
    fn part1real() {
        let real_input = include_str!("day2realinput.txt");
        println!("{}", part1solution(real_input));
        assert_eq!(part1solution(real_input), 402);
    }

    #[test]
    fn part2example() {
        let example_input = include_str!("day2exampleinput.txt");
        assert_eq!(part2solution(example_input), 4);
    }

    #[test]
    fn part2real() {
        let real_input = include_str!("day2realinput.txt");
        println!("{}", part2solution(real_input));
        assert_eq!(part2solution(real_input), 455);
    }
}

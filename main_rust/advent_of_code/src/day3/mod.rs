use regex::Regex;

fn part1solution(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    //let mut results = vec![];
    let mut sum = 0;
    for (_, [first_operand, second_operand]) in re.captures_iter(input).map(|c| c.extract()) {
        //results.push((first_operand.parse::<i32>(), second_operand.parse::<u64>().unwrap_or(continue), line));
        sum += first_operand.parse::<i32>().unwrap() * second_operand.parse::<i32>().unwrap();
    }
    //println!("{:#?}", results);
    sum
}

fn part2solution(input: &str) -> i32 {
    let re = Regex::new(r"(?:mul\((\d+),(\d+)\))|(?:(do\(\))())|(?:(don't\(\))())").unwrap();
    //let mut results = vec![];
    let mut sum = 0;
    let mut doo = true;
    for (_, [first_operand, second_operand]) in re.captures_iter(input).map(|c| c.extract()) {
        //results.push((first_operand.parse::<i32>(), second_operand.parse::<u64>().unwrap_or(continue), line));
        if first_operand == "do()" {doo = true}
        else if first_operand == "don't()" {doo = false} 
        else if doo {   
            sum += first_operand.parse::<i32>().unwrap() * second_operand.parse::<i32>().unwrap();
        }
    }
    //println!("{:#?}", results);
    sum
}


#[cfg(test)]
mod tests {
    use crate::day3::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("day3input.example")), 161);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("day3input.real")));
        assert_eq!(part1solution(include_str!("day3input.real")), 170068701);
    }

    #[test]
    fn part2example() {
        assert_eq!(part2solution("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"), 48);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("day3input.real")));
        assert_eq!(part2solution(include_str!("day3input.real")), 78683433);
    }
}

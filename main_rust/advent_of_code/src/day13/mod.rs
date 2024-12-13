use itertools::Itertools;
use nalgebra::{matrix, Matrix2};
use regex::Regex;

fn try_usize(num: f64) -> Option<usize> {
    let lemma = 0.001;
    let usize_sum: usize = num.round() as usize;
    if num.round() < num + lemma && num.round() > num - lemma {
        return Some(usize_sum);
    }
    None
}

fn part1solution(input: &str) -> usize {
    let re = Regex::new(r"Button A\: X\+([0-9]+), Y\+([0-9]+)\nButton B\: X\+([0-9]+), Y\+([0-9]+)\nPrize\: X\=([0-9]+), Y\=([0-9]+)").unwrap();
    let mut total_sum = 0;
    for (a_x, a_y, b_x, b_y, prize_x, prize_y) in re.captures_iter(input).map(|c| c.extract().1).map(|captives: [&str; 6]| captives.iter().map(|captive| captive.parse::<f64>().unwrap()).collect_tuple().unwrap()) {
        // PRIZE = iA + jB
        let mat1 = Matrix2::new(a_x, a_y, b_x, b_y);
        let res = matrix![prize_x, prize_y] * mat1.try_inverse().unwrap(); 
        let Some(a) = try_usize(res[0]) else {continue;};
        let Some(b) = try_usize(res[1]) else {continue;};
        total_sum += a * 3 + b
    }
    total_sum
}
fn part2solution(input: &str) -> usize {
    let re = Regex::new(r"Button A\: X\+([0-9]+), Y\+([0-9]+)\nButton B\: X\+([0-9]+), Y\+([0-9]+)\nPrize\: X\=([0-9]+), Y\=([0-9]+)").unwrap();
    let mut total_sum = 0;
    for (a_x, a_y, b_x, b_y, prize_x, prize_y) in re.captures_iter(input).map(|c| c.extract().1).map(|captives: [&str; 6]| captives.iter().map(|captive| captive.parse::<f64>().unwrap()).collect_tuple().unwrap()) {
        // PRIZE = iA + jB
        let mat1 = Matrix2::new(a_x, a_y, b_x, b_y);
        let res = matrix![prize_x + 10000000000000.0, prize_y + 10000000000000.0] * mat1.try_inverse().unwrap(); 
        let Some(a) = try_usize(res[0]) else {continue;};
        let Some(b) = try_usize(res[1]) else {continue;};
        total_sum += a * 3 + b
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use crate::day13::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("input.example")), 480);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("input.real")));
        assert_eq!(part1solution(include_str!("input.real")), 1433460);
    }
    
    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("input.real")));
        assert_eq!(part2solution(include_str!("input.real")), 71493195288102);
    }
}
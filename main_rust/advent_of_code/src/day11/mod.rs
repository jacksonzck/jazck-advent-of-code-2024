use std::{sync::LazyLock, usize};

use memoize::memoize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Hash)]
struct StoneGroup {
    stones: Vec<Stone>
}

impl StoneGroup {
    fn blink_n(self, n: usize) -> StoneGroup {
        let mut stoners = self.stones;
        for _ in 0..n {
            stoners = stoners.iter().map(|stone| stone.blink()).flatten().collect()
        }
        StoneGroup {stones: stoners}
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Stone {
    number: usize
}

impl Stone {
    fn blink(self) -> Vec<Stone> {
        match self.number {
            0 => vec![Stone {number: 1}],
            digits if digits.to_string().len() % 2 == 0 => vec![Stone {number: digits.to_string()[..digits.to_string().len() / 2].parse().unwrap()}, Stone {number: digits.to_string()[digits.to_string().len() / 2..].parse().unwrap()}],
            number => vec![Stone {number: number * 2024}]
        }
    }

    fn blink_n(self, cheatsheet: &Vec<Vec<StoneGroup>>, number: usize) -> StoneGroup {
        if number == 0 {
            StoneGroup {stones: vec![self]}
        } else if self.number < 10 {
            if number < 25 {
                cheatsheet[number][self.number].clone()
            } else {
                StoneGroup { stones: cheatsheet[24][self.number].stones.iter().map(|stone| stone.blink_n(cheatsheet, number - 24).stones).flatten().collect()}
            }
        } else {
            StoneGroup{ stones: StoneGroup {stones: self.blink()}.stones.iter().map(|stone| stone.blink_n(cheatsheet, number - 1).stones).flatten().collect()}
        }
    }

    fn count_n(self, cheatsheet: &Vec<Vec<StoneGroup>>, number: usize) -> usize {
        if number == 0 {
            1
        } else if self.number < 10 {
            if number < 39 {
                cheatsheet[number][self.number].stones.len()
            } else {
                cheatsheet[38][self.number].stones.iter().map(|stone| stone.count_n(cheatsheet, number - 38)).sum()
            }
        } else {
            self.blink().iter().map(|stone| stone.count_n(cheatsheet, number - 1)).sum()
        }
    }

    fn count_n_par(self, cheatsheet: &Vec<Vec<StoneGroup>>, number: usize) -> usize {
        if number == 0 {
            1
        } else if self.number < 10 {
            if number < 39 {
                cheatsheet[number][self.number].stones.len()
            } else {
                cheatsheet[38][self.number].stones.iter().map(|stone| stone.count_n(cheatsheet, number - 38)).sum()
            }
        } else {
            self.blink().par_iter().map(|stone| stone.count_n_par(cheatsheet, number - 1)).sum()
        }
    }
}



fn part1solution(input: &str) -> usize {
    let stone_groupie = StoneGroup {stones: input.split_whitespace().map(|stone| Stone {number: stone.parse().unwrap()}).collect()};
    stone_groupie.blink_n(25).stones.len()
}

pub fn part2solution(input: &str, n: usize) -> usize {
    let mut cheatsheet: Vec<Vec<StoneGroup>> = vec![];
    let mut first_cheats = vec![];
    for  stonenum in 0..10 {
        first_cheats.push(StoneGroup {stones: vec![Stone {number: stonenum}]});
    }
    cheatsheet.push(first_cheats);
    for iterations in 1..39 {
        let mut cheats: Vec<StoneGroup> = vec![];
        for stonenum in 0..10 {
            cheats.push(cheatsheet[iterations - 1][stonenum].clone().blink_n(1));
        }
        cheatsheet.push(cheats);
    }
    input.split_whitespace().map(|stone| Stone {number: stone.parse().unwrap()}.count_n(&cheatsheet, n)).sum()
}



pub fn part2solution_par(input: &str, n: usize) -> usize {
    let mut cheatsheet: Vec<Vec<StoneGroup>> = vec![];
    let mut first_cheats = vec![];
    for  stonenum in 0..10 {
        first_cheats.push(StoneGroup {stones: vec![Stone {number: stonenum}]});
    }
    cheatsheet.push(first_cheats);
    for iterations in 1..39 {
        let mut cheats: Vec<StoneGroup> = vec![];
        for stonenum in 0..10 {
            cheats.push(cheatsheet[iterations - 1][stonenum].clone().blink_n(1));
        }
        cheatsheet.push(cheats);
    }
    

    input.split_whitespace().map(|stone| Stone {number: stone.parse().unwrap()}.count_n_par(&cheatsheet, n)).sum()
}

fn make_cheatsheet() -> Vec<Vec<StoneGroup>> {
    let mut cheatsheet: Vec<Vec<StoneGroup>> = vec![];
    let mut first_cheats = vec![];
    for  stonenum in 0..10 {
        first_cheats.push(StoneGroup {stones: vec![Stone {number: stonenum}]});
    }
    cheatsheet.push(first_cheats);
    for iterations in 1..39 {
        let mut cheats: Vec<StoneGroup> = vec![];
        for stonenum in 0..10 {
            cheats.push(cheatsheet[iterations - 1][stonenum].clone().blink_n(1));
        }
        cheatsheet.push(cheats);
    }
    cheatsheet
}

/*static COOL_CHEATSHEET: LazyLock<Vec<Vec<StoneGroup>>> = LazyLock::new(|| make_cheatsheet());

pub fn part2solution_cool(input: &str, n: usize) -> usize {
    let mut cheatsheet: Vec<Vec<StoneGroup>> = vec![];
    let mut first_cheats = vec![];
    for  stonenum in 0..10 {
        first_cheats.push(StoneGroup {stones: vec![Stone {number: stonenum}]});
    }
    cheatsheet.push(first_cheats);
    for iterations in 1..39 {
        let mut cheats: Vec<StoneGroup> = vec![];
        for stonenum in 0..10 {
            cheats.push(cheatsheet[iterations - 1][stonenum].clone().blink_n(1));
        }
        cheatsheet.push(cheats);
    }

    #[memoize]
    fn count_n_cool(stone: Stone, number: usize) -> usize {
        if number == 0 {
            1
        } else if stone.number < 10 {
            if number < 39 {
                COOL_CHEATSHEET[number][stone.number].stones.len()
            } else {
                COOL_CHEATSHEET[38][stone.number].stones.iter().map(|stone| count_n_cool(*stone, number - 38)).sum()
            }
        } else {
            stone.blink().par_iter().map(|stone| count_n_cool(*stone, number - 1)).sum()
        }
    }

    input.split_whitespace().map(|stone| count_n_cool(Stone {number: stone.parse().unwrap()}, n)).sum()
}

*/
pub fn part2solution_coolest(input: &str, n: usize) -> usize {
    fn blink(number: usize) -> Vec<usize> {
        match number {
            0 => vec![1],
            digits if digits.to_string().len() % 2 == 0 => vec![digits.to_string()[..digits.to_string().len() / 2].parse().unwrap(), digits.to_string()[digits.to_string().len() / 2..].parse().unwrap()],
            number => vec![number * 2024]
        }
    }
    #[memoize]
    fn count_n_cool(stone: usize, number: usize) -> usize {
        if number == 0 {
            1
        } else {
            blink(stone).par_iter().map(|stone| count_n_cool(*stone, number - 1)).sum()
        }
    }
    input.split_whitespace().map(|stone| count_n_cool(stone.parse().unwrap(), n)).sum()
}

#[cfg(test)]
mod tests {
    use crate::day11::{part1solution, part2solution, part2solution_coolest, part2solution_par};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("input.example")), 55312);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("input.real")));
        assert_eq!(part1solution(include_str!("input.real")), 212655);
    }
    
    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("input.example"), 25), 55312);
    }
    
    #[test]
    fn part2real2() {
        println!("{}", part2solution(include_str!("input.real"), 75));
        //assert_eq!(part2solution(include_str!("input.real")), 1609);
    }

    #[test]
    fn part2real3() {
        println!("{}", part2solution_par(include_str!("input.real"), 75));
        //assert_eq!(part2solution(include_str!("input.real")), 1609);
    }

    #[test]
    fn part2real4() {
        assert_eq!(part2solution_coolest(include_str!("input.real"), 75), 253582809724830);
    }
}

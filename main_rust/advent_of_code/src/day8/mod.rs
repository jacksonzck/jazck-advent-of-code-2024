use std::collections::HashSet;
use itertools::{Itertools};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Antenna {
    frequency: char,
    location: Location
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Antinode {
    frequency: char,
    location: Location
}

impl Antenna {
    fn find_antinode(self, other: Antenna) -> Option<(Antinode, Antinode)> {
        if self.location == other.location {
            return None;
        }
        let x_diff = self.location.x - other.location.x;
        let y_diff = self.location.y - other.location.y;
        Some((Antinode {location: Location {x: self.location.x + x_diff, y: self.location.y + y_diff}, frequency: self.frequency},
        Antinode {location: Location {x: other.location.x - x_diff, y: other.location.y - y_diff}, frequency: self.frequency}))
    }

    fn find_cool_antinode(self, other: Antenna, max_location: Location) -> Vec<Antinode> {
        let mut new_vec = vec![];
        let x_diff = self.location.x - other.location.x;
        let y_diff = self.location.y - other.location.y;
        let mut new_anti = Antinode {location: Location {x: self.location.x, y: self.location.y}, frequency: self.frequency};
        while new_anti.location.in_bounds(max_location) {
            new_vec.push(new_anti);
            new_anti = Antinode {location: Location {x: new_anti.location.x + x_diff, y: new_anti.location.y + y_diff}, frequency: self.frequency}
        }
        let mut new_anti = Antinode {location: Location {x: other.location.x, y: other.location.y}, frequency: self.frequency};
        while new_anti.location.in_bounds(max_location) {
            new_vec.push(new_anti);
            new_anti = Antinode {location: Location {x: new_anti.location.x - x_diff, y: new_anti.location.y - y_diff}, frequency: self.frequency}
        }
        new_vec
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Location {
    x: i64,
    y: i64
}

impl Location {
    fn distance(self, other: Location) -> i64 {
        TryInto::<i64>::try_into(self.x.abs_diff(other.x)).unwrap() + TryInto::<i64>::try_into(self.y.abs_diff(other.y)).unwrap()
    }
    fn in_bounds (self, max_location: Location) -> bool {
        return self.x <= max_location.x && self.y <= max_location.y && self.x >= 0 && self.y >= 0;
    }
}

fn part1solution(input: &str) -> usize {
    let mut antennas = HashSet::new();
    for (line_number, line) in input.split_whitespace().enumerate() {
        for (char_number, character) in line.char_indices() {
            match character {
                '.' => (),
                _ => {antennas.insert(Antenna {frequency: character, location: Location { x: char_number.try_into().unwrap(), y: line_number.try_into().unwrap() }});}
            };
        }
    }
    let max_x = TryInto::<i64>::try_into(input.split_whitespace().last().unwrap().len()).unwrap() - 1;
    let max_y = TryInto::<i64>::try_into(input.split_whitespace().count()).unwrap() - 1;
    let max_location = Location {x: max_x, y: max_y};
    let antinodes = antennas.iter().combinations(2).filter(|a| a[0].frequency == a[1].frequency).map(|a| a[0].find_antinode(*a[1])).filter(|a| a.is_some()).map(|a| [a.unwrap().0, a.unwrap().1]).flatten().filter(|node| node.location.in_bounds(max_location)).map(|a| a.location).unique();
    let get_vecced: Vec<_> = antinodes.clone().collect();
    for (line_number, line) in input.split_whitespace().enumerate() {
        for (char_number, character) in line.char_indices() {
            if get_vecced.contains(&Location {x: char_number.try_into().unwrap(), y: line_number.try_into().unwrap()}) {
                print!("#");
            } else {
                print!("{}", character);
            }
        }
        println!("");
    }
    antinodes.count()
}

fn part2solution(input: &str) -> usize {
    let mut antennas = HashSet::new();
    for (line_number, line) in input.split_whitespace().enumerate() {
        for (char_number, character) in line.char_indices() {
            match character {
                '.' => (),
                _ => {antennas.insert(Antenna {frequency: character, location: Location { x: char_number.try_into().unwrap(), y: line_number.try_into().unwrap() }});}
            };
        }
    }
    let max_x = TryInto::<i64>::try_into(input.split_whitespace().last().unwrap().len()).unwrap() - 1;
    let max_y = TryInto::<i64>::try_into(input.split_whitespace().count()).unwrap() - 1;
    let max_location = Location {x: max_x, y: max_y};
    let antinodes = antennas.iter().combinations(2).filter(|a| a[0].frequency == a[1].frequency).map(|a| a[0].find_cool_antinode(*a[1], max_location)).flatten().map(|a| a.location).unique();
    let get_vecced: Vec<_> = antinodes.clone().collect();
    for (line_number, line) in input.split_whitespace().enumerate() {
        for (char_number, character) in line.char_indices() {
            if get_vecced.contains(&Location {x: char_number.try_into().unwrap(), y: line_number.try_into().unwrap()}) {
                print!("#");
            } else {
                print!("{}", character);
            }
        }
        println!("");
    }
    antinodes.count()
}

#[cfg(test)]
mod tests {
    use crate::day8::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("day8input.example")), 14);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("day8input.real")));
        assert_eq!(part1solution(include_str!("day8input.real")), 259);
    }

    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("day8input.example")), 34);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("day8input.real")));
        assert_eq!(part2solution(include_str!("day8input.real")), 927);
    }
}
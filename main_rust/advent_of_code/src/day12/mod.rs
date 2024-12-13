use std::{collections::HashSet, hash::Hash};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Location {
    x: usize,
    y: usize
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Plant {
    location: Location,
    form: char
}

#[derive(EnumIter, Debug, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}


impl Plant {
    fn find_adjacent_plants(&self, garden: &Vec<Vec<Plant>>) -> Vec<Plant> {
        let mut adjacent_plants = vec![];
        for direction in Direction::iter() {
            let (next_node_y, next_node_x) = match direction {
                Direction::UP if self.location.y > 0 => (self.location.y - 1, self.location.x),
                Direction::RIGHT if self.location.x < garden[0].len() - 1 => (self.location.y, self.location.x + 1),
                Direction::DOWN if self.location.y < garden.len() - 1 => (self.location.y + 1, self.location.x),
                Direction::LEFT if self.location.x > 0 => (self.location.y, self.location.x - 1),
                _ => continue
            };
            adjacent_plants.push(garden[next_node_y][next_node_x]);
        }
        adjacent_plants
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Region {
    plants: Vec<Plant>,
    form: char
}

impl Region {
    fn new(plant: Plant) -> Region {
        Region {plants: vec![plant], form: plant.form}
    }
    fn grow(self, garden: &Vec<Vec<Plant>>) -> Region {
        let mut frontier = self.plants.clone();
        let mut seen = self.plants.clone();
        while frontier.len() > 0 {
            let jeff = frontier.par_iter().map(|f| f.find_adjacent_plants(garden)).flatten().filter(|plant| !seen.contains(&plant) && plant.form == self.form);
            frontier = jeff.collect();
            frontier = frontier.iter().unique().map(|plant| *plant).collect();
            seen.extend(frontier.clone());
        }
        Region { plants: seen.iter().unique().map(|plant| *plant).collect(), form: self.form }
    }
    fn calculate_cost(self, garden: &Vec<Vec<Plant>>) -> usize {
        let mut perimeter = 0;
        for plant in &self.plants {
            perimeter += 4 - plant.find_adjacent_plants(garden).iter().filter(|o_plant| o_plant.form == self.form).count();
            //println!("Plant {:#?}\nPerimeter {:#?}", plant, plant.find_adjacent_plants(garden).iter().filter(|o_plant| o_plant.form == self.form).count());
        }
        perimeter * self.plants.len()
    }

    fn calculate_cost_cool(self, garden: &Vec<Vec<Plant>>) -> usize {
        todo!()
    }
}

fn part1solution(input: &str) -> usize {
    let mut garden = vec![];
    for (lineno, line) in input.split_whitespace().enumerate() {
        let mut garden_row = vec![];
        for (charno, character) in line.char_indices() {
            garden_row.push(Plant {location: Location {x: charno, y: lineno}, form: character});
        }
        garden.push(garden_row);
    }
    let mut scanned_plants = HashSet::new();
    let mut regions = vec![];
    for plantrow in &garden {
        for plant in plantrow {
            if scanned_plants.contains(plant) {
                continue;
            }
            let cool_region = Region::new(*plant);
            let cool_region = cool_region.grow(&garden);
            let plants = cool_region.plants.clone();
            for planter in &plants {
                scanned_plants.insert(planter.clone());
            }
            regions.push(cool_region);
        }
    }
    //println!("{:#?}", regions[0].clone().calculate_cost(&garden));
    regions.iter().map(|region| region.clone().calculate_cost(&garden)).sum()
    //todo!()
}
fn part2solution(input: &str) -> usize {
    todo!()
}
#[cfg(test)]
mod tests {
    use crate::day12::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("input.example")), 1930);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("input.real")));
        assert_eq!(part1solution(include_str!("input.real")), 1433460);
    }
    
    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("input.example")), 55312);
    }
    
    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("input.real")));
        //assert_eq!(part2solution(include_str!("input.real")), 1609);
    }
}
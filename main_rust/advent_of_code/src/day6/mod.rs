use std::{collections::HashSet, thread};
use rayon::prelude::*;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Location {
    x: usize,
    y: usize,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    location: Location,
    direction: Direction,
}

impl Guard {
    fn move_body(self, map: &Vec<Vec<Cell>>) -> Option<Guard> {
        let (move_try_x, move_try_y) = match self.direction {
            Direction::NORTH if self.location.y > 0 => (self.location.x, self.location.y - 1),
            Direction::EAST => (self.location.x + 1, self.location.y),
            Direction::SOUTH => (self.location.x, self.location.y + 1),
            Direction::WEST if self.location.x > 0 => (self.location.x - 1, self.location.y),
            _ => return None,
        };
        match map
            .get(move_try_y)
            .and_then(|e: &Vec<Cell>| e.get(move_try_x))
        {
            Some(Cell::EMPTY) => Some(Guard {
                location: Location {
                    x: move_try_x,
                    y: move_try_y,
                },
                direction: self.direction,
            }),
            Some(Cell::WALL) => match self.direction {
                Direction::NORTH => Some(Guard {
                    location: Location {
                        x: self.location.x,
                        y: self.location.y,
                    },
                    direction: Direction::EAST,
                }),
                Direction::EAST => Some(Guard {
                    location: Location {
                        x: self.location.x,
                        y: self.location.y,
                    },
                    direction: Direction::SOUTH,
                }),
                Direction::SOUTH => Some(Guard {
                    location: Location {
                        x: self.location.x,
                        y: self.location.y,
                    },
                    direction: Direction::WEST,
                }),
                Direction::WEST => Some(Guard {
                    location: Location {
                        x: self.location.x,
                        y: self.location.y,
                    },
                    direction: Direction::NORTH,
                }),
            },
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cell {
    WALL,
    EMPTY,
}

fn part1solution(input: &str) -> usize {
    let mut guard = None;
    let mut map = vec![];
    for (line_number, line) in input.split_whitespace().enumerate() {
        let mut map_line = vec![];
        for (character_number, character) in line.char_indices() {
            map_line.push(match character {
                '.' => Cell::EMPTY,
                '#' => Cell::WALL,
                '^' => {
                    guard = Some(Guard {
                        location: Location {
                            x: character_number,
                            y: line_number,
                        },
                        direction: Direction::NORTH,
                    });
                    Cell::EMPTY
                }
                _ => panic!(),
            });
        }
        map.push(map_line);
    }
    let mut visited_locations = HashSet::new();
    while guard.is_some() {
        visited_locations.insert(guard.unwrap().location);
        guard = guard.clone().unwrap().move_body(&map);
    }
    visited_locations.len()
}

fn find_visited_locations(input: &str) -> HashSet<Location> {
    let mut guard = None;
    let mut map = vec![];
    for (line_number, line) in input.split_whitespace().enumerate() {
        let mut map_line = vec![];
        for (character_number, character) in line.char_indices() {
            map_line.push(match character {
                '.' => Cell::EMPTY,
                '#' => Cell::WALL,
                '^' => {
                    guard = Some(Guard {
                        location: Location {
                            x: character_number,
                            y: line_number,
                        },
                        direction: Direction::NORTH,
                    });
                    Cell::EMPTY
                }
                _ => panic!(),
            });
        }
        map.push(map_line);
    }
    let mut visited_locations = HashSet::new();
    while guard.is_some() {
        visited_locations.insert(guard.unwrap().location);
        guard = guard.clone().unwrap().move_body(&map);
    }
    visited_locations
}

fn part2solution(input: &str) -> i32 {
    let mut guard = None;
    let mut map = vec![];
    for (line_number, line) in input.split_whitespace().enumerate() {
        let mut map_line = vec![];
        for (character_number, character) in line.char_indices() {
            map_line.push(match character {
                '.' => Cell::EMPTY,
                '#' => Cell::WALL,
                '^' => {
                    guard = Some(Guard {
                        location: Location {
                            x: character_number,
                            y: line_number,
                        },
                        direction: Direction::NORTH,
                    });
                    Cell::EMPTY
                }
                _ => panic!(),
            });
        }
        map.push(map_line);
    }
    let guard_starting_position = Location {
        x: guard.unwrap().location.x,
        y: guard.unwrap().location.y,
    };

    let mut stuck_counts = 0;
    let default_visited_locations = find_visited_locations(input);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if (Location { x: x, y: y }) == guard_starting_position {
                continue;
            }
            if !default_visited_locations.contains(&(Location { x: x, y: y })) {
                continue;
            }
            let mut cooler_map = map.clone();
            if cooler_map[y][x] == Cell::WALL {
                continue;
            } else {
                cooler_map[y][x] = Cell::WALL;
            }
            let mut cooler_guard = guard.clone();
            let mut seen_guards = HashSet::new();
            while cooler_guard.is_some() {
                seen_guards.insert(cooler_guard.unwrap());
                cooler_guard = cooler_guard.clone().unwrap().move_body(&cooler_map);
                if cooler_guard.is_some_and(|cooler_guard| seen_guards.contains(&cooler_guard)) {
                    stuck_counts += 1;
                    break;
                }
            }
        }
    }
    stuck_counts
}

fn part2solution_speeed(input: &str) -> usize {
    let mut guard = None;
    let mut map = vec![];
    for (line_number, line) in input.split_whitespace().enumerate() {
        let mut map_line = vec![];
        for (character_number, character) in line.char_indices() {
            map_line.push(match character {
                '.' => Cell::EMPTY,
                '#' => Cell::WALL,
                '^' => {
                    guard = Some(Guard {
                        location: Location {
                            x: character_number,
                            y: line_number,
                        },
                        direction: Direction::NORTH,
                    });
                    Cell::EMPTY
                }
                _ => panic!(),
            });
        }
        map.push(map_line);
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::day6::{part1solution, part2solution, part2solution_speeed};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("day6input.example")), 41);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("day6input.real")));
        assert_eq!(part1solution(include_str!("day6input.real")), 4454);
    }

    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("day6input.example")), 6);
    }

    #[test]
    fn part2examplefast() {
        assert_eq!(part2solution_speeed(include_str!("day6input.example")), 6);
    }

    #[test]
    fn part2realfaster() {
        assert_eq!(part2solution_speeed(include_str!("day6input.real")), 1503);
    }

    #[test]
    fn part2real() {
        //println!("{}", part2solution(include_str!("day6input.real")));
        assert_eq!(part2solution(include_str!("day6input.real")), 1503);
    }
}

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Location {
    x: usize,
    y: usize,
    ltype: Ltype
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Ltype {
    WALL,
    SPACE
}

#[derive(EnumIter, Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Self::EAST => Direction::WEST,
            Self::NORTH => Direction::SOUTH,
            Self::SOUTH => Direction::NORTH,
            Self::WEST => Direction::EAST
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    location: Location,
    score: usize,
    direction: Direction,
}

impl Node {
    fn get_adjacent_nodes(self, maze: &Vec<Vec<Location>>) -> Vec<Node> {
        let mut cool_nodes = vec![];
        for direction in Direction::iter() {
            let (x, y) = match direction {
                Direction::EAST => (self.location.x + 1, self.location.y),
                Direction::NORTH => (self.location.x, self.location.y - 1),
                Direction::SOUTH => (self.location.x, self.location.y + 1),
                Direction::WEST => (self.location.x - 1, self.location.y)
            };
            if maze[y][x].ltype == Ltype::SPACE {
                let score = match direction {
                    any if any == self.direction => self.score + 1,
                    any if any == self.direction.opposite() => self.score + 2001,
                    _ => self.score + 1001,
                };
                cool_nodes.push(Node { location: maze[y][x], score, direction});
            }
        }
        
        cool_nodes
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct CoolNode {
    location: Location,
    score: usize,
    direction: Direction,
    path_taken: Vec<Location>
}

impl CoolNode {
    fn get_adjacent_nodes(self, maze: &Vec<Vec<Location>>) -> Vec<CoolNode> {
        let mut cool_nodes = vec![];
        for direction in Direction::iter() {
            let (x, y) = match direction {
                Direction::EAST => (self.location.x + 1, self.location.y),
                Direction::NORTH => (self.location.x, self.location.y - 1),
                Direction::SOUTH => (self.location.x, self.location.y + 1),
                Direction::WEST => (self.location.x - 1, self.location.y)
            };
            if maze[y][x].ltype == Ltype::SPACE {
                let score = match direction {
                    any if any == self.direction => self.score + 1,
                    any if any == self.direction.opposite() => self.score + 2001,
                    _ => self.score + 1001,
                };
                if self.path_taken.contains(&maze[y][x]) {
                    continue;
                }
                let mut path_taken = self.path_taken.clone();
                path_taken.push(maze[y][x]);
                cool_nodes.push(CoolNode { location: maze[y][x], score, direction, path_taken});
            }
        }
        
        cool_nodes
    }
}


fn part1solution(input: &str) -> usize {
    let mut maze = vec![];
    let (mut start_x, mut start_y): (usize, usize) = (0, 0);
    let (mut end_x, mut end_y): (usize, usize) = (0, 0);
    for (lineno, line) in input.split_whitespace().enumerate() {
        let mut mazeline = vec![];
        for (charno, char) in line.char_indices() {
            match char {
                '#' => mazeline.push(Location {x: charno, y: lineno, ltype: Ltype::WALL}),
                '.' => mazeline.push(Location {x: charno, y: lineno, ltype: Ltype::SPACE}),
                'S' => {
                    mazeline.push(Location {x: charno, y: lineno, ltype: Ltype::SPACE});
                    (start_x, start_y) = (charno, lineno);
                },
                'E' => {
                    mazeline.push(Location {x: charno, y: lineno, ltype: Ltype::SPACE});
                    (end_x, end_y) = (charno, lineno);
                }
                _ => panic!()
            }
        }
        maze.push(mazeline);
    }
    let mut frontier = vec![Node {location: Location {x: start_x, y: start_y, ltype: Ltype::SPACE}, score: 0, direction: Direction::EAST}];
    let mut visited = HashSet::new();
    let ending_node = loop {
        frontier.sort_by(|a, ayy| ayy.score.cmp(&a.score));
        let next_node = frontier.pop().unwrap();
        if visited.contains(&next_node.location) {
            continue;
        }
        if next_node.location.x == end_x && next_node.location.y == end_y {
            break next_node;
        }
        visited.insert(next_node.location);
        for new_node in next_node.get_adjacent_nodes(&maze) {
            if !visited.contains(&new_node.location) {
                frontier.push(new_node);
            }
        }
    };
    ending_node.score
}

fn part2solution(input: &str) -> usize {
    let mut maze = vec![];
    let (mut start_x, mut start_y): (usize, usize) = (0, 0);
    let (mut end_x, mut end_y): (usize, usize) = (0, 0);
    for (lineno, line) in input.split_whitespace().enumerate() {
        let mut mazeline = vec![];
        for (charno, char) in line.char_indices() {
            match char {
                '#' => mazeline.push(Location {x: charno, y: lineno, ltype: Ltype::WALL}),
                '.' => mazeline.push(Location {x: charno, y: lineno, ltype: Ltype::SPACE}),
                'S' => {
                    mazeline.push(Location {x: charno, y: lineno, ltype: Ltype::SPACE});
                    (start_x, start_y) = (charno, lineno);
                },
                'E' => {
                    mazeline.push(Location {x: charno, y: lineno, ltype: Ltype::SPACE});
                    (end_x, end_y) = (charno, lineno);
                }
                _ => panic!()
            }
        }
        maze.push(mazeline);
    }
    let mut frontier = vec![CoolNode {location: Location {x: start_x, y: start_y, ltype: Ltype::SPACE}, score: 0, direction: Direction::EAST, path_taken: vec![Location {x: start_x, y: start_y, ltype: Ltype::SPACE}]}];
    let mut bestest_paths: Vec<CoolNode> = vec![];
    let mut visited: HashMap<(Location, Direction), usize> = HashMap::new();
    let best_paths = loop {
        frontier.sort_by(|a, ayy| ayy.score.cmp(&a.score));
        let next_node = frontier.pop().unwrap();
        if visited.get(&(next_node.location, next_node.direction)).is_some_and(|f| f != &next_node.score) {
            continue;
        }
        if bestest_paths.len() > 0 && bestest_paths[0].score < next_node.score {
            break bestest_paths;
        }
        visited.insert((next_node.location, next_node.direction), next_node.score);
        if next_node.location.x == end_x && next_node.location.y == end_y {
            bestest_paths.push(next_node);
            continue;
        }
        for new_node in next_node.get_adjacent_nodes(&maze) {
            if visited.get(&(new_node.location, new_node.direction)).is_some_and(|f| f != &new_node.score) {
                continue;
            }
            frontier.push(new_node);
        }
    };
    best_paths.iter().map(|f| f.path_taken.clone()).flatten().unique().count()
}

#[cfg(test)]
mod tests {
    use crate::day16::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("input.example")), 7036);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("input.real")));
        assert_eq!(part1solution(include_str!("input.real")), 94444);
    }
    
    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("input.example")), 45);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("input.real")));
        assert_eq!(part2solution(include_str!("input.real")), 502);
    }
}
use itertools::Itertools;
use strum::IntoEnumIterator;

use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node {
    x: usize,
    y: usize,
    height: usize
}

#[derive(EnumIter, Debug, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

impl Node {
    fn find_higher(&self, node_map: &Vec<Vec<Node>>) -> Vec<Node> {
        let mut adjacents = vec![];
        for direction in Direction::iter() {
            let (next_node_y, next_node_x) = match direction {
                Direction::UP if self.y > 0 => (self.y - 1, self.x),
                Direction::RIGHT if self.x < node_map[0].len() - 1 => (self.y, self.x + 1),
                Direction::DOWN if self.y < node_map.len() - 1 => (self.y + 1, self.x),
                Direction::LEFT if self.x > 0 => (self.y, self.x - 1),
                _ => continue
            };
            let next_node = node_map[next_node_y][next_node_x];
            if next_node.height == self.height + 1 {
                adjacents.push(next_node);
            }
        }

        adjacents
    }

    fn find_highest(self, node_map: &Vec<Vec<Node>>) -> Vec<Node> {
        let mut nodes = vec![self];
        let mut count = self.height;
        while count < 9 {
            let mut new_nodes = vec![];
            for node in nodes.clone() {
                new_nodes.extend(node.find_higher(node_map));
            }
            nodes = new_nodes.into_iter().unique().collect();
            count += 1;
        }
        nodes
    }

    fn cool_find_highest(self, node_map: &Vec<Vec<Node>>) -> Vec<Node> {
        let mut nodes = vec![self];
        let mut count = self.height;
        while count < 9 {
            let mut new_nodes = vec![];
            for node in nodes.clone() {
                new_nodes.extend(node.find_higher(node_map));
            }
            nodes = new_nodes;
            count += 1;
        }
        nodes
    }
}

fn part1solution(input: &str) -> usize {
    let mut topograph = vec![];
    for (lineno, line) in input.split_whitespace().enumerate() {
        let mut topoline = vec![];
        for (charno, character) in line.char_indices() {
            topoline.push(Node {x: charno, y: lineno, height: character.to_digit(10).unwrap().try_into().unwrap()})
        }
        topograph.push(topoline);
    }
    topograph.iter().flatten().filter(|node| node.height == 0).map(|node| node.find_highest(&topograph).iter().count()).sum()
}


fn part2solution(input: &str) -> usize { 
    let mut topograph = vec![];
    for (lineno, line) in input.split_whitespace().enumerate() {
        let mut topoline = vec![];
        for (charno, character) in line.char_indices() {
            topoline.push(Node {x: charno, y: lineno, height: character.to_digit(10).unwrap().try_into().unwrap()})
        }
        topograph.push(topoline);
    }
    topograph.iter().flatten().filter(|node| node.height == 0).map(|node| node.cool_find_highest(&topograph).iter().count()).sum()
}

#[cfg(test)]
mod tests {
    use crate::day10::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("input.example")), 36);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("input.real")));
        assert_eq!(part1solution(include_str!("input.real")), 754);
    }

    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("input.example")), 81);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("input.real")));
        assert_eq!(part2solution(include_str!("input.real")), 1609);
    }
}

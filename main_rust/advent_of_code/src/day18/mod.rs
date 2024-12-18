use std::collections::HashSet;

use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
struct Node {
    x: usize, 
    y: usize,
    steps: usize
}

impl Node {
    fn get_adjacent(self, gridsize: usize) -> Vec<Node> {
        let mut adjacents = vec![];
        if self.x > 0 {
            adjacents.push(Node { x: self.x - 1, y: self.y, steps: self.steps + 1})
        }
        if self.x < gridsize - 1 {
            adjacents.push(Node { x: self.x + 1, y: self.y, steps: self.steps + 1})
        }
        if self.y > 0 {
            adjacents.push(Node { x: self.x, y: self.y - 1, steps: self.steps + 1})
        }
        if self.y < gridsize - 1 {
            adjacents.push(Node { x: self.x, y: self.y + 1, steps: self.steps + 1})
        }
        adjacents
    }
}

fn part1solution(input: &str, gridsize: usize, chunk: usize) -> usize {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; gridsize]; gridsize];
    for (lineno, line) in input.split_whitespace().enumerate() {
        let linear: Vec<&str> = line.split(',').collect();
        if lineno < chunk {
            grid[linear[1].parse::<usize>().unwrap()][linear[0].parse::<usize>().unwrap()] = 1;
        }
    }
    let (start_x, start_y): (usize, usize) = (0, 0);
    let (goal_x, goal_y): (usize, usize) = (gridsize - 1, gridsize - 1);
    let mut frontier = vec![Node {x: start_x, y: start_y, steps: 0}];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let final_node: Node = loop {
        frontier.sort_by(|node, node_other| node_other.steps.cmp(&node.steps));
        //println!("{:?} {:?}", frontier, visited);
        let explore_node = frontier.pop().unwrap();
        if visited.contains(&(explore_node.x, explore_node.y)) {
            continue;
        }
        if explore_node.x == goal_x && explore_node.y == goal_y {
            break explore_node;
        }
        visited.insert((explore_node.x, explore_node.y));
        frontier.extend(explore_node.get_adjacent(gridsize).iter().filter(|p| grid[p.y][p.x] == 0 && !visited.contains(&(p.x, p.y))));
    };
    final_node.steps
}

fn part2solution(input: &str, gridsize: usize) -> &str {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; gridsize]; gridsize];
    for (lineno, line) in input.split_whitespace().enumerate() {
        let linear: Vec<&str> = line.split(',').collect();
        grid[linear[1].parse::<usize>().unwrap()][linear[0].parse::<usize>().unwrap()] = lineno + 1;
    }
    let (start_x, start_y): (usize, usize) = (0, 0);
    let (goal_x, goal_y): (usize, usize) = (gridsize - 1, gridsize - 1);
    let mut i = 0;
    fn loopy(i: usize, start_x: usize, start_y: usize, goal_x: usize, goal_y: usize, grid: &Vec<Vec<usize>>, gridsize: usize) -> Option<Node> {
        let mut frontier = vec![Node {x: start_x, y: start_y, steps: 0}];
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            let final_node: Option<Node> = loop {
                frontier.sort_by(|node, node_other| node_other.steps.cmp(&node.steps));
                //println!("{:?} {:?}", frontier, visited);
                let Some(explore_node) = frontier.pop() else {
                    return None
                };
                if visited.contains(&(explore_node.x, explore_node.y)) {
                    continue;
                }
                if explore_node.x == goal_x && explore_node.y == goal_y {
                    break Some(explore_node);
                }
                visited.insert((explore_node.x, explore_node.y));
                frontier.extend(explore_node.get_adjacent(gridsize).iter().filter(|p| (grid[p.y][p.x] == 0 || grid[p.y][p.x] > i) && !visited.contains(&(p.x, p.y))));
            };
            final_node
    }
    while loopy(i, start_x, start_y, goal_x, goal_y, &grid, gridsize).is_some() {
        i += 1;
    }
    input.split_whitespace().nth(i - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day18::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("input.example"), 7, 12), 22);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("input.real"), 71, 1024));
        assert_eq!(part1solution(include_str!("input.real"), 71, 1024), 94444);
    }
    
    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("input.example"), 7), "6,1");
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("input.real"), 71));
        assert_eq!(part2solution(include_str!("input.real"), 71), "16,44");
    }
}
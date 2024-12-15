use std::{hash::Hash, usize};
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

impl Direction {
    fn new_direction(direct: char) -> Direction {
        match direct {
            '^' => Direction::UP,
            '<' => Direction::LEFT,
            'v' => Direction::DOWN,
            '>' => Direction::RIGHT,
            _ => panic!()
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Object {
    x: usize, 
    y: usize,
    object_type: ObjectType
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum ObjectType {
    WALL,
    BOX
}

impl Object {
    fn push(&mut self, warehouse: &mut Vec<Vec<Option<Object>>>, direction: Direction) -> bool {
        if self.object_type == ObjectType::WALL {
            return false;
        }
        let (goto_y, goto_x) = match direction {
            Direction::UP if self.y > 0 => (self.y - 1, self.x),
            Direction::DOWN if self.y < warehouse.len() - 1 => (self.y + 1, self.x),
            Direction::LEFT if self.x > 0 => (self.y, self.x - 1),
            Direction::RIGHT if self.x < warehouse[0].len() - 1 => (self.y, self.x + 1),
            _ => return false,
        };
        let Some(mut pushed) = warehouse[goto_y][goto_x] else {
            warehouse[self.y][self.x] = None;
            self.x = goto_x;
            self.y = goto_y;
            warehouse[self.y][self.x] = Some(*self);
            return true;
        };
        if pushed.push(warehouse, direction) {
            warehouse[self.y][self.x] = None;
            self.x = goto_x;
            self.y = goto_y;
            warehouse[self.y][self.x] = Some(*self);
            return true;
        }
        false
    }
    
    fn gps(self) -> usize {
        if self.object_type == ObjectType::BOX {
            return self.y * 100 + self.x;
        } 
        return 0;
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct WideObject {
    x: usize, 
    y: usize,
    object_type: WideObjectType
}

impl WideObject {
    fn push(&mut self, warehouse: &mut Vec<Vec<WideObject>>, direction: Direction) -> bool {

        if self.object_type == WideObjectType::WALL {
            return false;
        } else if self.object_type == WideObjectType::RIGHTBOX {
            let mut jeff = warehouse[self.y][self.x - 1];
            return jeff.push(warehouse, direction);
        } else if self.object_type == WideObjectType::SPACE {
            return true;
        }
        match direction {
            Direction::UP => {
                if self.try_push(warehouse, &direction) {
                    let mut jeff = warehouse[self.y - 1][self.x];
                    assert!(jeff.push(warehouse, direction));
                    let mut joe = warehouse[self.y - 1][self.x + 1];
                    assert!(joe.push(warehouse, direction));
                    self.move_to(warehouse, self.x, self.y - 1);
                    return true;
                }
                false
            },
            Direction::DOWN => {
                if self.try_push(warehouse, &direction) {
                    let mut jeff = warehouse[self.y + 1][self.x];
                    assert!(jeff.push(warehouse, direction));
                    if jeff.object_type != WideObjectType::LEFTBOX {
                        let mut joe = warehouse[self.y + 1][self.x + 1];
                        assert!(joe.push(warehouse, direction));
                    }
                    self.move_to(warehouse, self.x, self.y + 1);
                    return true;
                }
                false
            },
            Direction::LEFT => {
                let mut jeff = warehouse[self.y][self.x - 1];
                if jeff.push(warehouse, direction) {
                    self.move_to(warehouse, self.x - 1, self.y);
                    return true;
                }
                false
            },
            Direction::RIGHT => {
                let mut jeff = warehouse[self.y][self.x + 2];
                if jeff.push(warehouse, direction) {
                    self.move_to(warehouse, self.x + 1, self.y);
                    return true;
                }
                false
            },
        }
    }

    fn try_push(self, warehouse: &Vec<Vec<WideObject>>, direction: &Direction) -> bool {
        if self.object_type == WideObjectType::WALL {
            return false;
        } else if self.object_type == WideObjectType::RIGHTBOX {
            let jeff = warehouse[self.y][self.x - 1];
            return jeff.try_push(warehouse, direction);
        } else if self.object_type == WideObjectType::SPACE {
            return true;
        } 
        match direction {
            Direction::UP => {
                let joe = warehouse[self.y - 1][self.x];
                let moe = warehouse[self.y - 1][self.x + 1];
                let toe = joe.try_push(warehouse, direction);
                return toe && moe.try_push(warehouse, direction)
            },
            Direction::DOWN => {
                let joe = warehouse[self.y + 1][self.x];
                let moe = warehouse[self.y + 1][self.x + 1];
                let toe = joe.try_push(warehouse, direction);
                return toe && moe.try_push(warehouse, direction)
            },
            _ => panic!()
        }
    }

    fn move_to(&mut self, warehouse: &mut Vec<Vec<WideObject>>, x: usize, y: usize) {
        assert!(self.object_type == WideObjectType::LEFTBOX);
        warehouse[self.y][self.x] = WideObject {x: self.x, y: self.y, object_type: WideObjectType::SPACE};
        warehouse[self.y][self.x + 1] = WideObject {x: self.x, y: self.y, object_type: WideObjectType::SPACE};
        self.x = x;
        self.y = y;
        warehouse[y][x] = *self;
        warehouse[y][x + 1] = WideObject {x: x + 1, y: y, object_type: WideObjectType::RIGHTBOX}
    }
    
    fn gps(self) -> usize {
        if self.object_type == WideObjectType::LEFTBOX {
            return self.y * 100 + self.x;
        } 
        return 0;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum WideObjectType {
    WALL,
    LEFTBOX,
    RIGHTBOX,
    SPACE
}


fn part1solution(input: &str) -> usize {
    let parts: Vec<_> = input.split("\n\n").collect();
    let warehouse_string = parts[0];
    let instruction_string = parts[1].replace('\n', "");
    let mut warehouse: Vec<Vec<Option<Object>>> = vec![];
    let (mut robot_x,mut robot_y): (usize, usize) = (0, 0);
    for (lineno, line) in warehouse_string.split_whitespace().enumerate() {
        let mut ware_line = vec![]; 
        for (charno, char) in line.char_indices() {
            match char {
                '#' => ware_line.push(Some(Object {x: charno, y: lineno, object_type: ObjectType::WALL})),
                'O' => ware_line.push(Some(Object {x: charno, y: lineno, object_type: ObjectType::BOX})),
                '.' => ware_line.push(None),
                '@' => {ware_line.push(None); robot_x = charno; robot_y = lineno;}, 
                _ => panic!(),
            }
        }
        warehouse.push(ware_line);
    }
    for direction in instruction_string.chars().map(|chararcter| Direction::new_direction(chararcter)) {
        let (pos_y, pos_x) = match direction {
            Direction::UP if robot_y > 0 => (robot_y - 1, robot_x),
            Direction::DOWN if robot_y < warehouse.len() - 1 => (robot_y + 1, robot_x),
            Direction::LEFT if robot_x > 0 => (robot_y, robot_x - 1),
            Direction::RIGHT if robot_x < warehouse[0].len() - 1 => (robot_y, robot_x + 1),
            _ => continue,
        };
        if warehouse[pos_y][pos_x].is_none() || warehouse[pos_y][pos_x].is_some_and(|mut object| object.push(&mut warehouse, direction)) {
            robot_x = pos_x;
            robot_y = pos_y;
            continue;
        }
    };
    warehouse.iter().map(|row| row.iter().filter(|f| f.is_some()).map(|f| f.unwrap().gps())).flatten().sum()
}

fn part2solution(input: &str) -> usize {
    let parts: Vec<_> = input.split("\n\n").collect();
    let warehouse_string = parts[0];
    let instruction_string = parts[1].replace('\n', "");
    let mut warehouse: Vec<Vec<WideObject>> = vec![];
    let (mut robot_x,mut robot_y): (usize, usize) = (0, 0);
    for (lineno, line) in warehouse_string.split_whitespace().enumerate() {
        let mut ware_line = vec![]; 
        for (charno, char) in line.char_indices() {
            match char {
                '#' => {
                    ware_line.push(WideObject {x: charno * 2, y: lineno, object_type: WideObjectType::WALL});
                    ware_line.push(WideObject {x: charno * 2 + 1, y: lineno, object_type: WideObjectType::WALL});
                },
                'O'  => {
                    ware_line.push(WideObject {x: charno * 2, y: lineno, object_type: WideObjectType::LEFTBOX});
                    ware_line.push(WideObject {x: charno * 2 + 1, y: lineno, object_type: WideObjectType::RIGHTBOX});
                },
                '.'  => {
                    ware_line.push(WideObject {x: charno * 2, y: lineno, object_type: WideObjectType::SPACE});
                    ware_line.push(WideObject {x: charno * 2 + 1, y: lineno, object_type: WideObjectType::SPACE});
                },
                '@' => {
                    robot_x = charno * 2; 
                    robot_y = lineno;
                    ware_line.push(WideObject {x: charno * 2, y: lineno, object_type: WideObjectType::SPACE});
                    ware_line.push(WideObject {x: charno * 2 + 1, y: lineno, object_type: WideObjectType::SPACE});
                },
                _ => panic!(),
            }
        }
        warehouse.push(ware_line);
    }
    for direction in instruction_string.chars().map(|chararcter| Direction::new_direction(chararcter)) {
        let (pos_y, pos_x) = match direction {
            Direction::UP if robot_y > 0 => (robot_y - 1, robot_x),
            Direction::DOWN if robot_y < warehouse.len() - 1 => (robot_y + 1, robot_x),
            Direction::LEFT if robot_x > 0 => (robot_y, robot_x - 1),
            Direction::RIGHT if robot_x < warehouse[0].len() - 1 => (robot_y, robot_x + 1),
            _ => continue,
        };
        let mut jeff = warehouse[pos_y][pos_x];
        if jeff.push(&mut warehouse, direction) {
            robot_x = pos_x;
            robot_y = pos_y;
            continue;
        }
    };
    warehouse.iter().map(|f| f.iter().map(|e| e.gps())).flatten().sum()
}


#[cfg(test)]
mod tests {
    use crate::day15::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("input.example")), 10092);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("input.real")));
        assert_eq!(part1solution(include_str!("input.real")), 1476771);
    }
    
    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("input.example")), 9021);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("input.real")));
        assert_eq!(part2solution(include_str!("input.real")), 1468005);
    }
}
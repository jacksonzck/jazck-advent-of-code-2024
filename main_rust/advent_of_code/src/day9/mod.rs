fn part1solution(input: &str) -> usize {
    let mut filesystem = vec![];
    let mut id: usize = 0;
    let mut file = true;
    for character in input.chars() {
        if file {
            filesystem.extend([Some(id)].repeat(character.to_string().parse().unwrap()));
            id += 1;
        } else {
            //println!("{}", character);
            filesystem.extend([None].repeat(character.to_string().parse().unwrap()));
        }
        file = !file;
    }
    let mut reverse_filesystem = filesystem.iter().enumerate().rev();
    let mut checksum = 0;
    let (mut reverse_head, mut reverse_value) = reverse_filesystem.next().unwrap();
    let mut last_consumed = reverse_head;
    for (index, value) in filesystem.iter().enumerate() {
        if index >= last_consumed {
            break;
        }
        match value {
            Some(v) => {
                checksum += index * v;
                print!("{}", v)
            }
            None => {
                while reverse_value.is_none() {
                    (reverse_head, reverse_value) = reverse_filesystem.next().unwrap();
                }
                if reverse_head < index {
                    break;
                }
                checksum += index * reverse_value.unwrap();
                last_consumed = reverse_head;
                //print!("{}", reverse_value.unwrap());
                (reverse_head, reverse_value) = reverse_filesystem.next().unwrap();
            }
        }
    }
    //println!("{:#?}", filesystem);
    checksum
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Block {
    id: Option<usize>,
    length: usize,
}

impl Block {
    fn split(self, other: Block) -> Option<(Block, Option<Block>)> {
        if self.id.is_some() {
            return None;
        }
        if self.length < other.length {
            return None;
        }
        if self.length == other.length {
            return Some((other, None));
        }
        Some((
            other,
            Some(Block {
                id: None,
                length: self.length - other.length,
            }),
        ))
    }
}

fn part2solution(input: &str) -> usize {
    let mut filesystem = vec![];
    let mut id: usize = 0;
    let mut file = true;
    for character in input.chars() {
        if file {
            filesystem.push(Block {
                id: Some(id),
                length: character.to_string().parse().unwrap(),
            });
            id += 1;
        } else {
            //println!("{}", character);
            filesystem.push(Block {
                id: None,
                length: character.to_string().parse().unwrap(),
            });
        }
        file = !file;
    }
    loop {
        let mut changes = 0;
        for (num, block) in filesystem
            .clone()
            .iter()
            .enumerate()
            .filter(|(_, block)| block.id.is_some()).rev()
        {
            let mut splota = None;
            let mut splot_index = 0;
            let mut splotb = None;
            //println!("{:#?}", block);
            for (split_index, to_split) in filesystem.iter().enumerate() {
                if split_index >= num {
                    break;
                }
                let splot = to_split.split(*block);
                if splot.is_none() {
                    continue;
                }
                splot_index = split_index;
                splota = Some(splot.unwrap().0);
                splotb = splot.unwrap().1;
                break;
            }
            if splota.is_none() {
                continue;
            }
            let last_block = filesystem.remove(num);
            filesystem.insert(num, Block {id: None, length: last_block.length});
            filesystem[splot_index] = splota.unwrap();
            if splotb.is_some() {
                filesystem.insert(splot_index + 1, splotb.unwrap());
            }
            changes += 1;
            break
        }
        if changes == 0 {
            break
        }
    }
    filesystem.into_iter().map(|block| [block.id].repeat(block.length)).flatten().enumerate().filter(|(_, block_id)| block_id.is_some()).map(|(index, block_id)| index * block_id.unwrap()).reduce(|acc, e| acc + e).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day9::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("day9input.example")), 1928);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("day9input.real")));
        assert_eq!(part1solution(include_str!("day9input.real")), 6432869891895);
    }

    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("day9input.example")), 2858);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("day9input.real")));
        assert_eq!(part2solution(include_str!("day9input.real")), 6467290479134);
    }
}

use regex::Regex;

fn part1solution(input: &str) -> usize {
    /*let mut split_input = input.split_whitespace();
    let mut rotated_input: Vec<Vec<char>> = vec![];
    for (lineno, line) in split_input.enumerate() {
        for (character_in, character) in line.char_indices() {
            let jeff = rotated_input.get(character_in);
            match jeff {
                Some(vector) => vector.to_owned().push(character),
                None => {rotated_input.push(vec![character]);},
            }
        }
    }
    let mut xmas_count = 0;
    let frontmas = Regex::new(r"XMAS").unwrap();
    let backmas = Regex::new(r"SAMX").unwrap();
    for line in input.split_whitespace() {
        xmas_count += frontmas.captures(line).map(|f| f.len()).unwrap_or(0);
        xmas_count += backmas.captures(line).map(|f| f.len()).unwrap_or(0);
    }
    for line in rotated_input {
        xmas_count += frontmas.captures(line.iter().collect::<String>().as_str()).map(|f| f.len()).unwrap_or(0);
        xmas_count += backmas.captures(line.iter().collect::<String>().as_str()).map(|f| f.len()).unwrap_or(0);
    } 
    xmas_count*/
    /*let mut lines = input.split_whitespace();
    let mut xmas_count = 0;
    for (linenumber, line) in lines.enumerate() {
        for (charnumber, character) in line.char_indices() {
            if character != 'X' {continue}
            
        }
    }*/
    let line_length = input.split_whitespace().into_iter().collect::<Vec<&str>>().get(0).unwrap().len();
    let binding = ".{".to_owned() + (line_length - 1).to_string().as_str() + "}";
    let down_left_char = binding.as_str();
    let binding = ".{".to_owned() + (line_length - 0).to_string().as_str() + "}";
    let down_char = binding.as_str();
    let binding = ".{".to_owned() + (line_length + 1).to_string().as_str() + "}";
    let down_right_char = binding.as_str();
    let forwards = Regex::new(r"XMAS").unwrap();
    let diagonal_up = Regex::new((r"S".to_owned() + down_left_char + "A" + down_left_char + "M" + down_left_char + "X").as_str()).unwrap();
    let up = Regex::new((r"S".to_owned() + down_char + "A" + down_char + "M" + down_char + "X").as_str()).unwrap();
    let diagonal_up_left = Regex::new((r"S".to_owned() + down_right_char + "A" + down_right_char + "M" + down_right_char + "X").as_str()).unwrap();
    let backwards = Regex::new(r"SAMX").unwrap();
    let diagonal_down = Regex::new((r"X".to_owned() + down_right_char + "M" + down_right_char + "A" + down_right_char + "S").as_str()).unwrap();
    let down = Regex::new((r"X".to_owned() + down_char + "M" + down_char + "A" + down_char + "S").as_str()).unwrap();
    let diagonal_down_other = Regex::new((r"X".to_owned() + down_left_char + "M" + down_left_char + "A" + down_left_char + "S").as_str()).unwrap();
    let mut xmas_count = 0;
    let stuff = input.replace("\n", "1").replace("\r", "").replace(" ", "");
    for regexes in vec![forwards, diagonal_up, up, diagonal_up_left, backwards, diagonal_down, down, diagonal_down_other] {
        let mut hot_stuff = stuff.clone();
        while hot_stuff.len() > 0 {
            match regexes.find(hot_stuff.as_str()) {
                Some(matche) => {hot_stuff.replace_range(0..(matche.start() + 1), "")},
                None => break,
            }
            xmas_count += 1;
        }
    }
    xmas_count
}

fn part2solution(input: &str) -> i32 {
    let line_length = input.split_whitespace().into_iter().collect::<Vec<&str>>().get(0).unwrap().len();
    let binding = ".{".to_owned() + (line_length - 1).to_string().as_str() + "}";
    let down_left_char = binding.as_str();
    let binding = ".{".to_owned() + (line_length - 0).to_string().as_str() + "}";
    let _down_char = binding.as_str();
    let binding = ".{".to_owned() + (line_length + 1).to_string().as_str() + "}";
    let _down_right_char = binding.as_str();
    println!("{}", down_left_char);
    let a = Regex::new((r"M.M".to_owned() + down_left_char + "A" + down_left_char + "S.S").as_str()).unwrap();
    let b = Regex::new((r"M.S".to_owned() + down_left_char + "A" + down_left_char + "M.S").as_str()).unwrap();
    let c = Regex::new((r"S.M".to_owned() + down_left_char + "A" + down_left_char + "S.M").as_str()).unwrap();
    let d = Regex::new((r"S.S".to_owned() + down_left_char + "A" + down_left_char + "M.M").as_str()).unwrap();
    let mut xmas_count = 0;
    let stuff = input.replace("\n", "1").replace("\r", "").replace(" ", "");
    for regexes in vec![a, b, c, d] {
        let mut hot_stuff = stuff.clone();
        while hot_stuff.len() > 0 {
            match regexes.find(hot_stuff.as_str()) {
                Some(matche) => {println!("{:#?}", matche); hot_stuff.replace_range(0..(matche.start() + 1), "")},
                None => break,
            }
            xmas_count += 1;
        }
    }
    xmas_count
}

#[cfg(test)]
mod tests {
    use crate::day4::{part1solution, part2solution};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("day4input.example")), 18);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("day4input.real")));
        assert_eq!(part1solution(include_str!("day4input.real")), 2573);
    }

    #[test]
    fn part2example() {
        assert_eq!(part2solution(include_str!("day4input.example_other")), 9);
    }

    #[test]
    fn part2real() {
        println!("{}", part2solution(include_str!("day4input.real")));
        assert_eq!(part2solution(include_str!("day4input.real")), 1850);
    }
}

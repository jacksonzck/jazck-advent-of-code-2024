use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
fn part1solution(input: &str, width: usize, height: usize) -> i64 {
    let re = Regex::new(r"p\=([0-9]+)\,([0-9]+) v\=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let mut spots: Vec<Vec<i64>> = vec![vec![0; width]; height];
    for (pos_x, pos_y, v_x, v_y) in re.captures_iter(input).map(|c| c.extract().1).map(|captives: [&str; 4]| captives.iter().map(|captive| captive.parse::<i64>().unwrap()).collect_tuple().unwrap()) {
        spots[((pos_y + (v_y * 100))).rem_euclid(height as i64) as usize][((pos_x + (v_x * 100))).rem_euclid(width as i64) as usize] += 1;
    } 
    for spot in spots.clone() {
        for spo in spot {
            print!("{}", spo);
        }
        println!("");
    }
    //println!("{:#?}", spots);
    let mut bl: i64 = 0;
    let mut tl: i64 = 0;
    let mut br: i64 = 0;
    let mut tr: i64 = 0;
    for (rownum, row) in spots.iter().enumerate() {
        for (colnum, col) in row.iter().enumerate() {
            let topside = rownum < (height - 1)/2;
            let botside = rownum >= (height + 1)/2;
            let leftside = colnum < (width - 1)/2;
            let rightside = colnum >= (width + 1)/2;
            if topside && leftside {
                print!("X");
                tl += col;
            } else if botside && leftside {
                print!("Y");
                bl += col;
            } else if topside && rightside {
                print!("A");
                tr += col;
            } else if botside && rightside {
                print!("B");
                br += col;
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!("{} {} {} {}", tl, bl, tr, br);
    bl * tl * br * tr
}


fn part2solution(input: &str, width: usize, height: usize) {
    let re = Regex::new(r"p\=([0-9]+)\,([0-9]+) v\=(-?[0-9]+),(-?[0-9]+)").unwrap();

    (0..99999).into_par_iter().for_each(|i| {
        {
            let mut spots: Vec<Vec<i64>> = vec![vec![0; width]; height];
            for (pos_x, pos_y, v_x, v_y) in re.captures_iter(input).map(|c| c.extract().1).map(|captives: [&str; 4]| captives.iter().map(|captive| captive.parse::<i64>().unwrap()).collect_tuple().unwrap()) {
                spots[((pos_y + (v_y * i))).rem_euclid(height as i64) as usize][((pos_x + (v_x * i))).rem_euclid(width as i64) as usize] += 1;
            }
            for (rownum, row) in spots.iter().enumerate() {
                let mut wow = vec![0; width];
                for (colnum, col) in row.iter().enumerate() {
                    if *col > 0 {
                        wow[colnum] += 1;
                        wow[width - colnum - 1] -= 1;
                    }
                }
                if wow.iter().filter(|f| **f != 0).count() > 10 {
                    return;
                }
            }
            println!("Second {}:", i);
            //let mut file = File::create(format!("Second {}:", i)).unwrap();
            for spot in spots.clone() {
            for spo in spot {
                print!("{}", spo);
                //file.write(format!("{}", spo).as_bytes()).unwrap();
            }
            println!();
            //file.write("\n".as_bytes()).unwrap();
            }
            //pause();
        } 
    }); 
}

fn part2solution_regex(input: &str, width: usize, height: usize) {
    let re = Regex::new(r"p\=([0-9]+)\,([0-9]+) v\=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let ore = Regex::new(r"^0*(?:[1-9]+0+)*$").unwrap();
    (0..9999).into_par_iter().for_each(|i| {
        {
            let mut spots: Vec<Vec<i64>> = vec![vec![0; width]; height];
            for (pos_x, pos_y, v_x, v_y) in re.captures_iter(input).map(|c| c.extract().1).map(|captives: [&str; 4]| captives.iter().map(|captive| captive.parse::<i64>().unwrap()).collect_tuple().unwrap()) {
                spots[((pos_y + (v_y * i))).rem_euclid(height as i64) as usize][((pos_x + (v_x * i))).rem_euclid(width as i64) as usize] += 1;
            }
            //println!("{}", spots.iter().map(|f| f.into_iter().join("")).join("\n"));
            //if spots.iter().map(|f| f.into_iter().join("")).any(|f| !ore.is_match(&f)) {
            //    return;
            //}
            let mut cool_second = false;
            //println!("Second {}:", i);
            //let mut file = File::create(format!("Second {}:", i)).unwrap();
            for spot in spots.clone() {
                let mut in_a_row = 0;
            for spo in spot {
                if spo > 0 {
                    in_a_row += 1;
                } else {
                    in_a_row = 0;
                }
                if in_a_row > 7 {
                    cool_second = true;
                }
                //print!("{}", spo);
            //    file.write(format!("{}", spo).as_bytes()).unwrap();
            }
            
            //println!();
            //file.write("\n".as_bytes()).unwrap();
            }
            if cool_second {
                println!("Second {}:", i);
                println!("{}", spots.iter().map(|f| f.into_iter().join("")).join("\n"));
            }
            //pause();
        } 
    }); 
}

#[cfg(test)]
mod tests {
    use crate::day14::{part1solution, part2solution, part2solution_regex};

    #[test]
    fn part1example() {
        assert_eq!(part1solution(include_str!("input.example"), 11, 7), 12);
    }

    #[test]
    fn part1real() {
        println!("{}", part1solution(include_str!("input.real"), 101, 103));
        assert_eq!(part1solution(include_str!("input.real"), 101, 103), 1433460);
    }
    
    #[test]
    fn part2example() {
        part2solution(include_str!("input.example"), 11, 7)
        //assert_eq!(part2solution(include_str!("input.example")), 480);
    }

    
    #[test]
    fn part2real() {
        part2solution_regex(include_str!("input.real"), 101, 103)
        //println!("{}", part2solution(include_str!("input.real")));
        //assert_eq!(part2solution(include_str!("input.real")), 71493195288102);
    }
}
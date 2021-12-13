
use std::ops::{Add, Sub};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use regex::Regex;


fn read_file(path: &PathBuf) -> Result<String, std::io::Error> {

    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    let mut output_string = String::with_capacity(file_size as usize);
    file.read_to_string(&mut output_string)?;

    return Ok(output_string);
}



#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Vec2 {

    x: u32,
    y: u32
}

impl Vec2 {

    pub fn new(x: u32, y: u32) -> Self {

        return Self{
            x,
            y
        };
    }
}

impl Add for Vec2 {
    
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        
        return Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        };
    }
}

impl Sub for Vec2 {

    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        
        return Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        };
    }
}

struct Map {

    size: Vec2,
    data: Vec<u32>
}

impl Map {

    pub fn new(size: Vec2) -> Self {

        let mut data = Vec::new();
        data.resize(size.x as usize * size.y as usize, 0);

        return Self{
            size,
            data
        };
    }

    pub fn get(&mut self, pos: Vec2) -> &mut u32 {

        return self.data.get_mut((self.size.x as usize* pos.y as usize) + pos.x as usize).unwrap();
    }
}

fn parse_input(input: &str) -> Vec<(Vec2, Vec2)> {

    let match_vec_pair: Regex = Regex::new(r#"(\d*),(\d*) -> (\d*),(\d*)"#).unwrap();
    let mut output = vec![];

    for line in input.lines().map(|line| line.trim()) {

        let captures = match_vec_pair.captures(line).unwrap();
        let x1 = captures.get(1).unwrap().as_str().parse().unwrap();
        let y1 = captures.get(2).unwrap().as_str().parse().unwrap();
        let x2 = captures.get(3).unwrap().as_str().parse().unwrap();
        let y2 = captures.get(4).unwrap().as_str().parse().unwrap();
        
        output.push((Vec2::new(x1, y1), Vec2::new(x2, y2)));
    }

    return output;
}


fn min_max(value: i64, min: i64, max: i64) -> i64 {

    return i64::min(max, i64::max(min, value));
}


fn calculate(input: &Vec<(Vec2, Vec2)>) -> u64 {

    let insert_line = |map: &mut Map, line: &(Vec2, Vec2)| {

        let mut current_position = line.0;
        
        let x_dir = min_max(line.1.x as i64 - line.0.x as i64, -1, 1);
        let y_dir = min_max(line.1.y as i64 - line.0.y as i64, -1, 1);
        
        loop {
        
            *map.get(current_position) += 1;
            
            current_position.x = (current_position.x as i64 + x_dir) as u32;
            current_position.y = (current_position.y as i64 + y_dir) as u32;
            
            if current_position == line.1 {
                
                *map.get(current_position) += 1;
                break;
            }
        }
    };

    let x_size = input.iter().flat_map(|lines| [lines.0.x, lines.1.x]).fold(0, |sum, x| if x > sum { x } else { sum }) + 1;
    let y_size = input.iter().flat_map(|lines| [lines.0.y, lines.1.y]).fold(0, |sum, y| if y > sum { y } else { sum }) + 1;

    let mut map = input.iter().fold(Map::new(Vec2::new(x_size, y_size)), |mut map, line| {insert_line(&mut map, line); map});

    let mut total = 0;
    for x in 0..x_size {
        for y in 0..y_size {

            if *map.get(Vec2::new(x, y)) >= 2 {

                total += 1;
            }
        }
    }

    return total;
}


fn measure<T>(func: impl FnOnce() -> T) -> (u128, T) {

    let before = std::time::Instant::now();
    let output = func();
    let after = std::time::Instant::now();

    return ((after - before).as_nanos(), output);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let filename = std::env::args().nth(1).unwrap_or("./input.txt".to_owned());
    let raw_input = read_file(&filename.into())?;
    let input = parse_input(&raw_input);

    let part_1 = measure(|| calculate(&input.iter().filter(|line| (line.0.x == line.1.x) || (line.0.y == line.1.y)).map(|lines| *lines).collect()));
    let part_2 = measure(|| calculate(&input));
    
    println!("part 1 | result: {}, time: {}", part_1.1, part_1.0);
    println!("part 2 | result: {}, time: {}", part_2.1, part_2.0);

    return Ok(());
}

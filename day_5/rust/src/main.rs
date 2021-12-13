
use std::ops::{Add, Sub};
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use regex::Regex;
use hashbrown::HashMap;


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

    type Map = HashMap<Vec2, u32>;
    let reduce_map = |mut a: Map, b: Map| -> Map {

        for (key, value) in a.iter_mut() {
            
            if let Some(b_value) = b.get(&key) {

                *value += *b_value;
            }
        }

        return a;
    };

    let insert_line = |map: &mut Map, line: &(Vec2, Vec2)| {

        let mut current_position = line.0;
        
        let x_dir = min_max(line.1.x as i64 - line.0.x as i64, -1, 1);
        let y_dir = min_max(line.1.y as i64 - line.0.y as i64, -1, 1);
        
        loop {
        
            *map.entry(current_position).or_insert(0) += 1;
            
            current_position.x = (current_position.x as i64 + x_dir) as u32;
            current_position.y = (current_position.y as i64 + y_dir) as u32;
            
            if current_position == line.1 {
                
                *map.entry(current_position).or_insert(0) += 1;
                break;
            }
        }
    };


    return input.iter()
        .fold(HashMap::new(), |mut map, line| {insert_line(&mut map, line); map})
        .iter().fold(0, |sum, (_, value)| if *value >= 2 { sum + 1 } else { sum }) as u64;
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

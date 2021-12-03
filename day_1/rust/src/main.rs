#![feature(array_windows)]

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;



fn read_file(path: PathBuf) -> Result<String, std::io::Error> {

    let mut file = File::open(path)?;
    let mut file_content = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut file_content)?;

    return Ok(file_content);
}

fn parse_input(input: String) -> Result<Vec<u32>, Box<dyn std::error::Error>> {

    let output = input.lines().into_iter().map(|line| line.trim().parse::<u32>()).collect::<Result<_, _>>()?;
    return Ok(output);
}

fn count_increase(input: &[u32]) -> u64 {

    return input.array_windows::<2>().skip(1).fold(0, |count, [a, b]| if *b > *a { count + 1 } else { count })
}

fn count_three_measurement_window_increase(input: &[u32]) -> u64 {

    let windows: Vec<u32> = input.array_windows::<3>().map(|[a, b, c]| a + b + c).collect();
    return count_increase(&windows);
}

fn main() {
    
    let raw_input = read_file("./input.txt".into()).expect("input read error");
    let input = parse_input(raw_input).expect("input parse error");
    
    println!("result part 1: {}", count_increase(&input));
    println!("result part 2: {}", count_three_measurement_window_increase(&input));

}

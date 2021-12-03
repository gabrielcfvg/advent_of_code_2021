
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use simple_error::simple_error;



fn read_file(path: PathBuf) -> Result<String, std::io::Error> {

    let mut file = File::open(path)?;
    let mut file_content = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut file_content)?;

    return Ok(file_content);
}



enum Command {

    Forward(u64),
    Up(u64),
    Down(u64)
}

fn parse_command(input: &str) -> Result<Command, Box<dyn std::error::Error>> {

    let parts: Vec<&str> = input.trim().split(' ').collect();
    let number = parts[1].parse::<u64>()?;
    
    let command = match parts[0] {
    
        "forward" => Command::Forward(number),
        "up" => Command::Up(number),
        "down" => Command::Down(number),
    
        _ => return Err(Box::new(simple_error!("command parsing error")))
    };
    
    return Ok(command);
}

fn parse_input(raw_input: &str) -> Result<Vec<Command>, Box<dyn std::error::Error>> {

    return raw_input.lines().map(|line| parse_command(line)).collect::<Result<_, _>>()
}

fn calculate_final_position_1(input: &Vec<Command>) -> (u64, u64) {

    let mut final_position = (0, 0);

    for command in input.iter() {

        match command {
            Command::Forward(number) => final_position.0 += number,
            Command::Up(number) => final_position.1 -= number,
            Command::Down(number) => final_position.1 += number,
        }
    }

    return final_position;
}

fn calculate_final_position_2(input: &Vec<Command>) -> (u64, u64) {

    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in input.iter() {

        match command {
            Command::Forward(number) => {
                position += number;
                depth += aim * number;
            },
            Command::Up(number) => aim -= number,
            Command::Down(number) => aim += number,
        }
    }

    return (position, depth);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let raw_input = read_file("./input.txt".into())?;
    let input = parse_input(&raw_input)?;

    let final_position_1 = calculate_final_position_1(&input);
    println!("part 1 result: {}", final_position_1.0 * final_position_1.1);

    let final_position_2 = calculate_final_position_2(&input);
    println!("part 2 result: {}", final_position_2.0 * final_position_2.1);

    return Ok(());
}

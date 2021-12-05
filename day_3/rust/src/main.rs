
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;



const LINE_SIZE: usize = 12;


fn read_file(path: PathBuf) -> Result<String, std::io::Error> {

    let mut file = File::open(path)?;
    let mut file_content = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut file_content)?;

    return Ok(file_content);
}

fn parse_input(input : &str) -> Vec<[u8; LINE_SIZE]> {

    let input_size = input.lines().count();
    let mut output = Vec::with_capacity(input_size);

    for line in input.lines() {

        let mut line_content = [0; LINE_SIZE];

        for (idx, ch) in line.trim().chars().enumerate() {
            
            line_content[idx] = (ch == '1') as u8;
        }

        output.push(line_content);
    }   

    return output;
}

fn count_bits(input: &Vec<[u8; LINE_SIZE]>) -> [u32; LINE_SIZE] {

    let mut bit_count: [u32; LINE_SIZE] = [0; LINE_SIZE];

    for line in input.iter() {

        for idx in 0..LINE_SIZE {

            bit_count[idx] += line[idx] as u32;
        }
    }

    return bit_count;
}

fn bits_to_number(input: &[u8; LINE_SIZE]) -> u64 {

    return input.iter().rev().enumerate().map(|(idx, bit)| u64::pow(2, idx as u32) * (*bit as u64)).sum();
}

fn calculate_part_1(input: &Vec<[u8; LINE_SIZE]>) -> u64 {

    let input_size = input.len();
    let bit_count = count_bits(input);

    let gamma_rate: u64 = bits_to_number(&bit_count.map(|number| (number >= (input_size / 2) as u32) as u8));
    let epsilon_rate = gamma_rate ^ (u64::MAX >> (64 - LINE_SIZE));

    return gamma_rate * epsilon_rate;
}

fn calculate_part_2(input: &Vec<[u8; LINE_SIZE]>) -> u64 {

    let filter_input = |filter_func: &mut dyn Fn(u32, usize) -> u8 | {
        
        let mut input = input.clone();
        let mut bit_count = count_bits(&input);
        let mut bit_idx = 0;

        while input.len() > 1 {

            let value_to_filter = filter_func(bit_count[bit_idx], input.len());
            input = filter_lines(input, value_to_filter, bit_idx);
            bit_count = count_bits(&input);
            bit_idx += 1;
        }

        return input[0];
    };

    let oxygen = bits_to_number(&filter_input(&mut |num, input_size| (!(num >= ((input_size as u32) - num))) as u8));
    let co2 = bits_to_number(&filter_input(&mut |num, input_size| (!(num < ((input_size as u32) - num))) as u8));

    return oxygen * co2;
}

fn filter_lines(mut input: Vec<[u8; LINE_SIZE]>, value_to_filter: u8, position: usize) -> Vec<[u8; LINE_SIZE]> {

    let mut new_idx = 0;
    let mut idx = 0;

    while idx < input.len() {
        
        if input[idx][position] != value_to_filter {
            
            input[new_idx] = input[idx];
            new_idx += 1;
        }

        idx += 1;
    }
    input.truncate(new_idx);

    return input;
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let raw_input = read_file("./input.txt".into())?;
    let input = parse_input(&raw_input);

    println!("part 1 result: {}", calculate_part_1(&input));
    println!("part 2 result: {}", calculate_part_2(&input));

    return Ok(());
}


use std::path::PathBuf;
use std::fs::File;
use std::io::Read;


fn read_file(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {

    let mut file = File::open(path)?;
    let mut output = String::with_capacity(file.metadata()?.len() as usize);
    file.read_to_string(&mut output)?;
    
    return Ok(output);
}


fn parse_input(input: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {

    return Ok(input.trim().split(",").map(|number| number.parse()).collect::<Result<_, _>>()?);
}


fn part_1(input: &Vec<u32>) -> u64 {

    let mut input = input.clone();
    input.sort();

    let destination = if input.len() % 2 == 0 {

        (input[input.len() / 2] + input[(input.len() / 2) + 1]) / 2
    }
    else {
        
        input[(input.len() / 2) + 1]
    };

    return input.iter().fold(0, |sum ,n| sum + (destination as i64 - *n as i64).abs() as u64);
}


fn part_2(input: &Vec<u32>) -> u64 {

    let avg = (input.iter().fold(0, |sum, num| sum + *num as u64) as u64 / input.len() as u64) as u32;

    let costs: Vec<u64> = [avg, avg + 1].iter().map(|avg| {
        
        input.iter().fold(0, |sum, num| {

            let distance = ((avg - 1) as i64 - *num as i64).abs() as u64;
            let triangle = (distance * (distance + 1)) / 2;
            return sum + triangle as u64;
        })
    }).collect();

    return *costs.iter().min().unwrap();
}


fn measure<T>(func: impl Fn() -> T) -> (u128, T) {

    let before = std::time::Instant::now();
    let output = func();
    let after = std::time::Instant::now();

    return ((after - before).as_nanos(), output);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let filename = std::env::args().nth(1).unwrap_or("./input.txt".to_owned());
    let raw_input = read_file(&filename.into())?;
    let input = parse_input(&raw_input)?;

    let part_1 = measure(|| part_1(&input));
    let part_2 = measure(|| part_2(&input));

    println!("part 1 | result: {}, time: {}", part_1.1, part_1.0);
    println!("part 2 | result: {}, time: {}", part_2.1, part_2.0);

    return Ok(());
}


use std::path::PathBuf;
use std::fs::File;
use std::io::Read;



fn read_file(path: &PathBuf) -> Result<String, std::io::Error> {

    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    let mut output_string = String::with_capacity(file_size as usize);
    file.read_to_string(&mut output_string)?;

    return Ok(output_string);
}

fn parse_input(input: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    return Ok(input.trim().split(",").map(|number| number.parse()).collect::<Result<_, _>>()?);
}


fn calculate(init_fishes: &Vec<u8>, day_count: u64) -> u64 {

    let mut fishes = vec![];
    fishes.resize(9, 0);

    for fish in init_fishes.iter() {

        fishes[(8 - *fish) as usize] += 1;
    }

    for _ in 0..day_count {

        fishes.rotate_right(1);
        fishes[2] += fishes[0];
    }

    return fishes.iter().sum();
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
    let input = parse_input(&raw_input)?;



    let part_1 = measure(|| calculate(&input, 80));
    let part_2 = measure(|| calculate(&input, 256));
    
    println!("part 1 | result: {}, time: {}", part_1.1, part_1.0);
    println!("part 2 | result: {}, time: {}", part_2.1, part_2.0);

    return Ok(());
}
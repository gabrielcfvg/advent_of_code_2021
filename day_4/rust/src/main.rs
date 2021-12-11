
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use arrayvec::ArrayVec;



const MATRICE_SIZE_LEN: usize = 5;



fn read_file(path: &PathBuf) -> Result<String, std::io::Error> {

    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    let mut output_string = String::with_capacity(file_size as usize);
    file.read_to_string(&mut output_string)?;

    return Ok(output_string);
}



#[derive(Debug, Clone)]
struct Board {

    pub columns: ArrayVec<ArrayVec<u16, MATRICE_SIZE_LEN>, MATRICE_SIZE_LEN>,
    pub rows: ArrayVec<ArrayVec<u16, MATRICE_SIZE_LEN>, MATRICE_SIZE_LEN>
}

impl Board {

    pub fn new(lines: &ArrayVec<ArrayVec<u16, MATRICE_SIZE_LEN>, MATRICE_SIZE_LEN>) -> Self {

        let mut columns = (0..MATRICE_SIZE_LEN).map(|_| ArrayVec::new()).collect::<ArrayVec<_, MATRICE_SIZE_LEN>>();
        let mut rows = (0..MATRICE_SIZE_LEN).map(|_| ArrayVec::new()).collect::<ArrayVec<_, MATRICE_SIZE_LEN>>();

        for row in 0..MATRICE_SIZE_LEN {
            for column in 0..MATRICE_SIZE_LEN {
                
                let value = lines[row][column];

                let column_insert_pos = columns[row].binary_search(&value).unwrap_or_else(|pos| pos);
                columns[row].insert(column_insert_pos, value);
                
                let row_insert_pos = rows[column].binary_search(&value).unwrap_or_else(|pos| pos);
                rows[column].insert(row_insert_pos, value);
            }
        }

        return Self{
            columns,
            rows
        };
    }
}


fn parse_input(input: &str) -> Result<(Vec<u16>, Vec<Board>), Box<dyn std::error::Error>> {

    let number_sequence = input.lines().nth(0).unwrap().split(',').map(|number| number.parse()).collect::<Result<_, _>>()?;
    let mut boards: Vec<Board> = vec![];

    {
        let mut current_board: ArrayVec<ArrayVec<u16, MATRICE_SIZE_LEN>, MATRICE_SIZE_LEN> = ArrayVec::new();

        for line in input.lines().skip(2).map(|line| line.trim()) {

            if line.len() == 0 {


                boards.push(Board::new(&current_board));

                current_board = ArrayVec::new();
            }
            else {

                current_board.push(line.split_ascii_whitespace().map(|line| line.parse().unwrap()).collect());
            }
        }
    }

    return Ok((number_sequence, boards));
}


fn process_board(board: &mut Board, number: u16) -> Option<u64> {

    for lines in [&mut board.rows, &mut board.columns].iter_mut() {

        for line in lines.iter_mut() {

            if let Ok(idx) = line.binary_search(&number) {

                line.remove(idx);

                if line.len() == 0 {
                    
                    return Some(lines.iter().map(|line| line.iter().fold(0, |state, number| state + *number as u64)).sum());
                }
            }
        }
    }

    return None;
}


fn calculate_part_1(input: &(Vec<u16>, Vec<Board>)) -> u64 {

    let mut boards = input.1.clone();
    for number in input.0.iter().map(|number| *number) {

        let result = boards.iter_mut().map(|board| process_board(board, number)).find(|result| result.is_some());

        if let Some(Some(sum)) = result {

            return sum * number as u64;
        }
    }

    unreachable!();
}


fn calculate_part_2(input: &(Vec<u16>, Vec<Board>)) -> u64 {

    let mut boards = input.1.clone();
    for number in input.0.iter().map(|number| *number) {
        
        if boards.len() == 1 {

            let sum = process_board(&mut boards[0], number);
            
            if let Some(sum) = sum {
                
                return sum * number as u64;
            }
        }
        else {

            boards = boards.drain(..)
                .map(|mut board| if matches!(process_board(&mut board, number), None) { Some(board) } else { None })
                .filter(|result| result.is_some())
                .map(|board| board.unwrap())
            .collect();
        }
    }

    unreachable!();
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let filename = "./input.txt";
    let raw_input = read_file(&filename.into())?;
    let input = parse_input(&raw_input)?;

    println!("part 1 result: {}", calculate_part_1(&input));
    println!("part 2 result: {}", calculate_part_2(&input));

    return Ok(());
}

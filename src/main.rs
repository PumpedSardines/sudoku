mod sudoku;
use std::fs;
use std::fs::File;
use std::io::Read;

// https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/
fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn main() {
    let v = get_file_as_byte_vec(&String::from("sudoku/hard.sudoku"));

    let mut byte_arr: [u8; 81] = [0; 81];

    for (i, val) in v.iter().enumerate() {
        byte_arr[i] = *val;
    }

    let mut hardest_sudoku = sudoku::Board::new_from_bytes(byte_arr).unwrap();

    hardest_sudoku.solve().unwrap();

    println!("{}", hardest_sudoku);
}

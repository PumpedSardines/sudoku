mod debug;
mod errors;
mod solve;

use rand::Rng;

#[derive(Clone, Copy)]
pub struct Board {
    pub board: [[Option<u8>; 9]; 9],
}

impl Board {
    #[allow(dead_code)]
    pub fn new() -> Board {
        return Board {
            board: [[None; 9]; 9],
        };
    }

    #[allow(dead_code)]
    pub fn new_from_bytes(bytes: [u8; 81]) -> Result<Board, errors::LoadFromBytesError> {
        let mut board = Board {
            board: [[None; 9]; 9],
        };

        for x in 0..9 {
            for y in 0..9 {
                let byte = bytes[x + y * 9];

                board.board[x][y] = match byte {
                    0 => None,
                    1..=9 => Some(byte),
                    _ => return Err(errors::LoadFromBytesError),
                };
            }
        }

        return Ok(board);
    }



    #[allow(dead_code)]
    pub fn new_solved() -> Board {
        let mut board = [[None; 9]; 9];

        // Since all three diagonal grids in the sudoku board have no impact on eachother
        // We can save a lot of time by placing random numbers in them

        for i in 0..3 {
            let mut pool = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

            for x in 0..3 {
                for y in 0..3 {
                    let rand_index = rand::thread_rng().gen_range(0..{ pool.len() });
                    board[x + 3 * i][y + 3 * i] = Some(pool.remove(rand_index));
                }
            }
        }

        println!("{}", debug::string_board(&board));

        if let Err(_) = solve::recursive_solve(&mut board, 0, 0) {
            eprintln!("Error: {}", "couldn't generate solved board");
            std::process::exit(1);
        }

        return Board { board };
    }



    #[allow(dead_code)]
    pub fn get_bytes(&self) -> [u8; 81] {
        let mut ret = [0; 81];

        for x in 0..9 {
            for y in 0..9 {
                ret[x + y * 9] = self.board[x][y].unwrap_or(0);
            }
        }

        return ret;
    }

    #[allow(dead_code)]
    pub fn solve(&mut self) -> Result<(), errors::SolveError> {
        if let Err(_) = solve::recursive_solve(&mut self.board, 0, 0) {
            return Err(errors::SolveError);
        }
    
        return Ok(());
    }
}

impl std::fmt::Display for Board {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", debug::string_board(&self.board))
    }
}
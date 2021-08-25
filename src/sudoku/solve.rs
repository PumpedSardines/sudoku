use rand::Rng;

// Recursivly solves a sudoku board
pub fn recursive_solve(board: &mut [[Option<u8>; 9]; 9], x: usize, y: usize) -> Result<(), ()> {
    // If last square is populated in advance we need to check and return
    if x == 8 && y == 8 && board[x][y] != None {
        return Ok(());
    }

    // Since this tile could be populated by a number we can skip it
    if board[x][y] != None {
        // Start by generating the new x and y values for next recursion iteration
        let new_x = match y != 8 {
            true => x,
            false => x + 1,
        };
        let new_y = match y != 8 {
            true => y + 1,
            false => 0,
        };

        // Call recursion and handle return
        if let Ok(_) = recursive_solve(board, new_x, new_y) {
            return Ok(());
        }
        return Err(());
    }

    // We start by generating a pool of numbers that could possibly populate the tile
    let mut pool = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Loop trough pool times since we want to check every number
    for _ in 0..pool.len() {
        // Essentially we try a random number until one works and we return the loop when we find a working number
        let rand_index = rand::thread_rng().gen_range(0..{ pool.len() });
        let num = pool.remove(rand_index); // Remove returns what it removes 🔥🔥🔥

        let can_be_placed = can_place_horizontal(board, num, x)
            && can_place_vertical(board, num, y)
            && can_place_grid(board, num, x, y);

        if can_be_placed {
            board[x][y] = Some(num);

            if x == 8 && y == 8 {
                // The pointer for number placement has reached it's end and we can return ok
                return Ok(());
            } else {
                // Generate the new x and y position for the next tile to place
                let new_x = match y != 8 {
                    true => x,
                    false => x + 1,
                };
                let new_y = match y != 8 {
                    true => y + 1,
                    false => 0,
                };



                // Call recursive, there's a chance however that the number we picked won't work down the line.
                // If nothing works this call will return an error, and we essentilly try a new number again
                if let Ok(_) = recursive_solve(board, new_x, new_y) {
                    return Ok(());
                }

                board[x][y] = None;
            }
        }
    }

    // This should never be returned if the board is solveable
    Err(())
}

/// Checks if a number can be placed on a ceirtain latitude
fn can_place_vertical(board: &mut [[Option<u8>; 9]; 9], num: u8, y: usize) -> bool {
    for i in 0..9 {
        match board[i][y] {
            None => {}
            Some(val) => {
                if val == num {
                    return false;
                }
            }
        }
    }

    return true;
}

/// Checks if a number can be placed on a ceirtain longitude
fn can_place_horizontal(board: &mut [[Option<u8>; 9]; 9], num: u8, x: usize) -> bool {
    return !board[x].iter().any(|v| {
        return match v {
            None => false,
            Some(val) => *val == num,
        };
    });
}

/// Checks if a number can be placed in the sudoku grid it's in
fn can_place_grid(board: &mut [[Option<u8>; 9]; 9], num: u8, x: usize, y: usize) -> bool {
    let x_square = x / 3;
    let y_square = y / 3;

    for x in 0..3 {
        for y in 0..3 {
            let num_exists = match board[x + x_square * 3][y + y_square * 3] {
                None => false,
                Some(val) => val == num,
            };

            if num_exists {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {

    #[test]
    fn can_place_horizontal() {
        assert_eq!(true, super::can_place_horizontal(&mut [[None; 9]; 9], 1, 0));
        assert_eq!(false, super::can_place_horizontal(&mut [
            [None, None, None, None, None, None, None, None, None],
            [None, None, Some(3), None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
        ], 3, 1));
    }

    #[test]
    fn can_place_vertical() {
        assert_eq!(true, super::can_place_vertical(&mut [[None; 9]; 9], 1, 0));
        assert_eq!(false, super::can_place_vertical(&mut [
            [None, None, None, None, None, None, None, None, None],
            [None, None, Some(3), None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
        ], 3, 2));
    }

    #[test]
    fn can_place_grid() {
        assert_eq!(true, super::can_place_grid(&mut [[None; 9]; 9], 1, 0, 0));
        assert_eq!(false, super::can_place_grid(&mut [
            [None, None, None, None, None, None, None, None, None],
            [None, None, Some(3), None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
        ], 3, 0, 0));
        assert_eq!(false, super::can_place_grid(&mut [
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, Some(3), None],
            [None, None, None, None, None, None, None, None, None],
        ], 3, 6, 6));
        assert_eq!(true, super::can_place_grid(&mut [
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, Some(3), None],
            [None, None, None, None, None, None, None, None, None],
        ], 3, 4, 6));
    }
}

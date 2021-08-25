pub fn string_board(board: &[[Option<u8>; 9]; 9]) -> String {
    let mut print_str = String::from("");
        
    for x in 0..9 {
        for y in 0..9 {

            if y != 0 && y % 3 == 0 {
                print_str.push_str("| ");
            }

            

            match board[x][y] {
                None => print_str.push_str("-"),
                Some(num) => print_str.push_str(&num.to_string()),
            }



            if y == 8 {
                print_str.push_str("\n");
                if x != 0 && x != 8 && (x + 1) % 3 == 0 {
                    print_str.push_str("---------------------\n");
                }
            } else  {
                print_str.push_str(" ");
            }
        }
    }

    return print_str;
}
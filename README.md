# Sudoku in Rust
A way to create, solve, save and load sudokus with rust.
```rs

fn main() {
    let sudoku_game = sudoku::new_solved();
}

```
If you want to solve a sudoku, you can load it in and the call the solve method
```rs

fn main() {
    let mut my_cool_sudoku = sudoku::Board::new_from_bytes(byte_arr).unwrap();

    my_cool_sudoku.solve().unwrap();

    println!("{}", my_cool_sudoku);
}

```
use text_io::scan;
use term_grid::{Grid, GridOptions, Direction, Filling};
use text_to_ascii_art::convert;
use game_of_life_logic as gol;

// Function to print the board in the terminal
pub fn get_grid_string(game: gol::GameOfLife) -> String{
    let board = game.board.clone();
    let size = game.size;
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    });

    for i in 0..size {
        for j in 0..size {
            if board[i][j] {
                grid.add(format!("X").into());
            } else {
                grid.add(format!(" ").into());
            }
        }
        grid.add_new_row();
    }
    return grid.fit_into_width(50).unwrap();
}

// Function to print a headline as ASCII art with text_to_ascii_art
pub fn print_headline(headline: &str) {
    let ascii_art = convert(headline);
    println!("{}", ascii_art);
}

// Function that allows the user to create a custom board with x and o's
pub fn user_creates_custom_board(size: usize) -> Vec<Vec<bool>> {
    let mut board = vec![vec![false; size]; size];
    println!("Please enter {} characters for each row. Use 'x' for a living cell and 'o' for a dead cell.", size);
    println!("You will have to enter {} rows. EXAMPLE: 'ooooo' ENTER 'oxxxo' ENTER 'ooooo' ENTER", size);
    for i in 0..size {
        let mut row;
        scan!("{}", row);
        for (j, c) in row.chars().enumerate() {
            if c == 'x' {
                board[i][j] = true;
            }
        }
    }
    return board;
}

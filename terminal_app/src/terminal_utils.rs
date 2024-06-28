use text_io::scan;
use text_to_ascii_art::convert;
use game_of_life_logic as gol;

// Function to print the board in the terminal, using the Grid library
pub fn get_grid_string(game: gol::GameOfLife) -> String {
    let mut result = String::new();
    for row in game.board.iter() {
        for &cell in row.iter() {
            if cell {
                result.push('🟩');
            } else {
                result.push('⬜');
            }
        }
        result.push('\n');
    }
    result
}

// Function to print a headline as ASCII art with text_to_ascii_art
pub fn print_headline(headline: &str) {
    let ascii_art = convert(headline.to_string());
    match ascii_art {
        Ok(art) => println!("{}", art),
        Err(e) => eprintln!("Error while converting to ASCII art: {}", e),
    }
}

// Function that allows the user to create a custom board with x and o's
pub fn user_creates_custom_board(size: usize) -> Vec<Vec<bool>> {
    let mut board = vec![vec![false; size]; size];
    println!("Please enter {} characters for each row. Use 'x' for a living cell and 'o' for a dead cell.", size);
    println!("You will have to enter {} rows. EXAMPLE: 'ooooo' ENTER 'oxxxo' ENTER 'ooooo' ENTER", size);
    for i in 0..size {
        let row: String;
        scan!("{}", row);
        for (j, c) in row.chars().enumerate() {
            if c == 'x' {
                board[i][j] = true;
            }
        }
    }
    return board;
}

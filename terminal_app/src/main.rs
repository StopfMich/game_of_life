use game_of_life_logic as gol;
use std::process::ExitCode;
use clearscreen::clear;
use text_io::scan;

mod terminal_utils;
use terminal_utils as tu;

fn main() -> ExitCode {
    loop {
        main_menu();
        clear();
        let size = select_size();
        clear();
        let sleep_time_in_millis = select_sleep_time();
        clear();
        let max_generation = select_max_generation();
        let use_custom_board = select_board();
        clear();
        let game;
        if use_custom_board {
            let board_from_user = tu::user_creates_custom_board(size);
            game = gol::create_custom_board(board_from_user, size);
            clear();
        } else {
            game = gol::create_random_board(size);
        }
        game_loop(game, sleep_time_in_millis, max_generation);
        clear();
        println!("Do you want to play again? (y/n)");
        let mut input;
        scan!("{}", input);
        if input != "y" {
            break;
        }
    }
    tu::print_headline("Thanks for playing!");
    ExitCode::SUCCESS
}

fn main_menu() {
    tu::print_headline("Game of Life");
    println!("In your terminal!");
    //wait for user input
    println!("Press enter to start the game");
    let mut garbage;
    scan!("{}", garbage);
}

fn select_size() -> usize {
    tu::print_headline("Select board size");
    let mut size;
    loop {
        println!("Please enter the size of the board (2-50): ");
        scan!("{}", size);
        if size > 1 && size <= 50 {
            break;
        } else {
            println!("Please enter a value between 2 and 50.");
        }
    }
    return size;
}

fn select_sleep_time() -> usize {
    tu::print_headline("Select sleep time");
    let mut sleep_time;
    loop {
        println!("Please enter the sleep time in milliseconds (1-1000): ");
        scan!("{}", sleep_time);
        if sleep_time > 0 && sleep_time <= 1000 {
            break;
        } else {
            println!("Please enter a value between 1 and 1000.");
        }
    }
    return sleep_time;
}

fn select_max_generation() -> usize {
    tu::print_headline("Select max generation");
    let mut max_generation;
    loop {
        println!("Please enter the max generation (1-1000): ");
        scan!("{}", max_generation);
        if max_generation > 0 && max_generation <= 1000 {
            break;
        } else {
            println!("Please enter a value between 1 and 1000.");
        }
    }
    return max_generation;
}

fn select_board() -> bool {
    tu::print_headline("Select board");
    println!("Do you want to create a custom board? (y/n)");
    let mut input;
    scan!("{}", input);
    return input == "y";
}

fn game_loop(game: gol::GameOfLife, sleep_time_in_millis: usize, max_generation: usize) {
    let end = false;
    let last_games: Vec;
    let current_generation = 0;
    while !end {
        let grid_string = tu::get_grid_string(game);
        println!("{}", grid_string);
        std::thread::sleep(std::time::Duration::from_millis(sleep_time_in_millis as u64));
        last_games.push(game);
        let updated_game = gol::update_board(game);
        end = gol::check_game_end(updated_game, vec![last_games], current_generation, max_generation);
        clear();
    }

    tu::print_headline("Game over!");
    println!("The game has ended after {} generations.", current_generation);
    println!("Last board:");
    let grid_string = tu::get_grid_string(game);
    println!("{}", grid_string);
    println!("Press enter to return to continue.");
    let mut garbage;
    scan!("{}", garbage);
}
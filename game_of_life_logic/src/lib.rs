use rand::prelude::*;

#[derive(Clone)]
pub struct GameOfLife {
    pub board: Vec<Vec<bool>>,
    pub size: usize
}

pub fn create_game(size: usize) -> GameOfLife {
    let board = vec![vec![false; size]; size];
    GameOfLife { board, size }
}

pub fn update_game(board: GameOfLife) -> GameOfLife {
    let new_game_of_life = update_board(board);
    return new_game_of_life;
}

pub fn create_custom_board(board: Vec<Vec<bool>>, size: usize) -> GameOfLife {
    GameOfLife { board, size: size.try_into().unwrap() }
}

pub fn create_random_board(size: usize) -> GameOfLife {
    let mut board = vec![vec![false; size]; size];
    let mut rng = rand::thread_rng();
    for i in 0..size {
        for j in 0..size {
            board[i][j] = rng.gen_bool(0.5);
        }
    }
    return GameOfLife { board, size }
}

pub fn insert_cell(game_of_life: GameOfLife, x: usize, y: usize) -> GameOfLife {
    let mut new_board = game_of_life.board.clone();
    new_board[x][y] = true;
    return GameOfLife { board: new_board, size: game_of_life.size }
}

fn check_cell(board: &GameOfLife, x: usize, y: usize) -> bool {
    let mut count = 0;
    let size = board.size as usize;
    let x = x as usize;

    // Check all 8 directions
    let directions = vec![
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    for (dx, dy) in directions {
        let mut nx = x as isize + dx;
        let mut ny = y as isize + dy;
        // check if point is on the border -> check the opposite side
        if nx < 0 {
            nx = size as isize - 1;
        } else if nx >= size as isize {
            nx = 0;
        }
        if ny < 0 {
            ny = size as isize - 1;
        } else if ny >= size as isize {
            ny = 0;
        }
        if board.board[nx as usize][ny as usize] {
            count += 1;
        }
    }
    return match count {
        2 => board.board[x][y],
        3 => true,
        _ => false
    }

}

pub fn update_board(old_game_of_life: GameOfLife) -> GameOfLife {
    let size = old_game_of_life.size;
    let mut new_board = vec![vec![false; size]; size]; //empty board
    for i in 0..size {
        for j in 0..size {
            new_board[i][j] = check_cell(&old_game_of_life, i, j);
        }
    }
    return create_custom_board(new_board, size);
}

// Function to check if the game is in a loop (up to 4 generations) or stable state
pub fn check_game_end(current_game: GameOfLife, last_games: Vec<GameOfLife>, current_generation: usize, max_generation: usize) -> bool {

    // delete all games except the last 4
    let mut last_four_games = last_games.clone();
    while last_four_games.len() > 4 {
        last_four_games.remove(0);
    }

    // max generation reached?
    if current_generation >= max_generation {
        return true;
    }

    // check if the game is in a loop
    for game in last_games.iter() {
        if game.board == current_game.board {
            return true;
        }
    }
    return false;
}

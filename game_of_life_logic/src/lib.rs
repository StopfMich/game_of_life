use rand::prelude::*;

#[derive(Clone)]
pub struct GameOfLife {
    pub board: Vec<Vec<bool>>,
    pub size: usize,
    pub game_history: Vec<GameOfLife>
}

pub fn create_game(size: usize) -> GameOfLife {
    let board = vec![vec![false; size]; size];
    let game_history = Vec::new();
    GameOfLife { board, size, game_history }
}

impl GameOfLife {

    pub fn update_game(mut self) -> GameOfLife {
        //let new_game_of_life = update_board(board);
        let size = self.size;
        let mut new_board = vec![vec![false; size]; size]; //empty board
        for i in 0..size {
            for j in 0..size {
                new_board[i][j] = self.clone().check_cell(i, j);
            }
        }
        self.board = new_board;
        self.game_history.push(self.clone());
        return self;
    }

    pub fn use_custom_board(mut self, board: Vec<Vec<bool>>, size: usize) -> GameOfLife {
        self.board = board;
        self.size = size.try_into().unwrap();
        return self;
    }

    pub fn use_random_board(mut self,size: usize) -> GameOfLife {
        let mut board = vec![vec![false; size]; size];
        let mut rng = rand::thread_rng();
        for i in 0..size {
            for j in 0..size {
                board[i][j] = rng.gen_bool(0.5);
            }
        }
        self.board = board;
        self.size = size;
        return self;
    }

    pub fn insert_cell(mut self, x: usize, y: usize) -> GameOfLife {
        self.board[x][y] = true;
        return self;
    }

    pub fn remove_cell(mut self, x: usize, y: usize) -> GameOfLife {
        self.board[x][y] = false;
        return self;
    }

    fn check_cell(self, x: usize, y: usize) -> bool {
        let mut count = 0;
        let size = self.size as usize;
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
            if self.board[nx as usize][ny as usize] {
                count += 1;
            }
        }
        return match count {
            2 => self.board[x][y],
            3 => true,
            _ => false
        }
    }

    pub fn check_stuck(mut self, current_generation: usize, max_generation: usize) -> bool {
        // delete all games except the last 4
        while self.game_history.len() > 4 {
            self.game_history.remove(0);
        }

        // max generation reached?
        if current_generation >= max_generation {
            return true;
        }

        // check if the game is in a loop
        for game in self.game_history.iter() {
            if game.board == self.board {
                return true;
            }
        }
        return false;
    }
}

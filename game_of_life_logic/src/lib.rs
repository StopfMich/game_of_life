use rand::prelude::*;
use std::collections::VecDeque;


#[derive(Clone)]
pub struct GameOfLife {
    pub board: Vec<Vec<bool>>,
    pub size: usize,
    pub game_history: VecDeque<u64>
}

pub fn create_game(size: usize) -> GameOfLife {
    let board = vec![vec![false; size]; size];
    let game_history = VecDeque::new();
    GameOfLife { board, size, game_history }
}

impl GameOfLife {

    pub fn update_game(&mut self) {
        let size = self.size;
        let mut new_board = vec![vec![false; size]; size];
        for i in 0..size {
            for j in 0..size {
                new_board[i][j] = self.check_cell(i, j); // Direkter Zugriff, kein Klonen
            }
        }
        self.board = new_board;
        // Fügen Sie hier die Logik hinzu, um den Zustand in der Historie zu speichern, ohne das gesamte Spiel zu klonen
        self.manage_history();
    }

        // Beispiel für eine optimierte Historienverwaltung
        fn manage_history(&mut self) {
            let state_hash = self.hash_state();
            self.game_history.push_front(state_hash);
            while self.game_history.len() > 4 {
                self.game_history.pop_back();
            }
        }

    pub fn use_custom_board(&mut self, board: Vec<Vec<bool>>, size: usize) {
        self.board = board;
        self.size = size.try_into().unwrap();
    }

    pub fn use_random_board(&mut self,size: usize) {
        let mut board = vec![vec![false; size]; size];
        let mut rng = rand::thread_rng();
        for i in 0..size {
            for j in 0..size {
                board[i][j] = rng.gen_bool(0.5);
            }
        }
        self.board = board;
        self.size = size;
    }

    pub fn insert_cell(&mut self, x: usize, y: usize) {
        self.board[x][y] = true;
    }

    pub fn remove_cell(&mut self, x: usize, y: usize) {
        self.board[x][y] = false;
    }

    fn check_cell(&self, x: usize, y: usize) -> bool {
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

    pub fn check_stuck(self, current_generation: usize, max_generation: usize) -> bool {
        // max generation reached?
        if current_generation >= max_generation {
            return true;
        }

        // check if the game is in a loop
        for i in 1..self.game_history.len() {
            if self.game_history[i] == self.game_history[0] {
                return true;
            }
        }

        return false;
    }

    fn hash_state(&self) -> u64 {
        const FNV_PRIME: u64 = 1099511628211;
        const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
    
        let mut hash = FNV_OFFSET_BASIS;
        for row in self.board.iter() {
            for &cell in row.iter() {
                hash ^= cell as u64;
                hash = hash.wrapping_mul(FNV_PRIME);
            }
        }
        return hash
    }
}

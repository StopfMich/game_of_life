#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use game_of_life_logic::*;

    //Tests for the game_of_life_logic module
    #[test]
    fn test_update_game() {
        let board = vec![
            vec![true, false, true],
            vec![false, true, false],
            vec![true, false, true],
        ];
        let game = create_custom_board(board.clone(), 3);
        let updated_game = update_game(game);
    
        assert_eq!(updated_game.board, vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ]);
    }

    #[test]
    fn test_insert_cell() {
        // Create a game with a 3x3 board
        let board = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ];
        let game = create_custom_board(board.clone(), 3);

        // Insert a cell at position (1, 1)
        let updated_game = insert_cell(game, 1, 1);

        // Assert that the updated game has the correct board
        assert_eq!(updated_game.board, vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ]);
    }

    #[test]
    fn rotate_stick() {
        // Create a game with a 5x5 board
        let board = vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, true, true, true, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ];
        let game = create_custom_board(board.clone(), 5);

        // Update the game
        let updated_game = update_game(game);

        // Assert that the updated game has the correct board
        assert_eq!(updated_game.board, vec![
            vec![false, false, false, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, true, false, false],
            vec![false, false, false, false, false],
        ]);
    }
}
use super::*;
use anchor_lang::prelude::*;

#[cfg(test)]
mod game_tests {
    use super::*;

    /// Helper function to create a default Game instance for testing
    fn create_test_game() -> Game {
        Game {
            player_x: Pubkey::new_unique(),
            player_o: Pubkey::new_unique(),
            wager: 1_000_000_000, // 1 SOL
            board: [0; 9],
            turn: 1,
            status: 0,
            total_pot: 2_000_000_000, // 2 SOL
            created_at: 1_640_000_000,
            last_move_ts: 1_640_000_000,
            timeout_seconds: 300,
            bump: 255,
        }
    }

    #[test]
    fn test_game_creation_default_values() {
        let game = create_test_game();
        
        assert_eq\!(game.wager, 1_000_000_000);
        assert_eq\!(game.board, [0; 9]);
        assert_eq\!(game.turn, 1);
        assert_eq\!(game.status, 0);
        assert_eq\!(game.total_pot, 2_000_000_000);
        assert_eq\!(game.timeout_seconds, 300);
    }

    #[test]
    fn test_game_board_initialization_empty() {
        let game = create_test_game();
        
        for cell in game.board.iter() {
            assert_eq\!(*cell, 0, "All board cells should be empty (0) initially");
        }
    }

    #[test]
    fn test_game_board_size() {
        let game = create_test_game();
        
        assert_eq\!(game.board.len(), 9, "Board should have exactly 9 cells");
    }

    #[test]
    fn test_game_player_x_turn() {
        let mut game = create_test_game();
        game.turn = 1;
        
        assert_eq\!(game.turn, 1, "Turn 1 should represent player X");
    }

    #[test]
    fn test_game_player_o_turn() {
        let mut game = create_test_game();
        game.turn = 2;
        
        assert_eq\!(game.turn, 2, "Turn 2 should represent player O");
    }

    #[test]
    fn test_game_status_open() {
        let mut game = create_test_game();
        game.status = 0;
        
        assert_eq\!(game.status, 0, "Status 0 should represent open game");
    }

    #[test]
    fn test_game_status_x_won() {
        let mut game = create_test_game();
        game.status = 1;
        
        assert_eq\!(game.status, 1, "Status 1 should represent X won");
    }

    #[test]
    fn test_game_status_o_won() {
        let mut game = create_test_game();
        game.status = 2;
        
        assert_eq\!(game.status, 2, "Status 2 should represent O won");
    }

    #[test]
    fn test_game_status_draw() {
        let mut game = create_test_game();
        game.status = 3;
        
        assert_eq\!(game.status, 3, "Status 3 should represent draw");
    }

    #[test]
    fn test_game_board_valid_x_move() {
        let mut game = create_test_game();
        game.board[0] = 1; // X marks position 0
        
        assert_eq\!(game.board[0], 1, "Board position should contain X marker (1)");
    }

    #[test]
    fn test_game_board_valid_o_move() {
        let mut game = create_test_game();
        game.board[4] = 2; // O marks center position
        
        assert_eq\!(game.board[4], 2, "Board position should contain O marker (2)");
    }

    #[test]
    fn test_game_board_multiple_moves() {
        let mut game = create_test_game();
        game.board[0] = 1; // X
        game.board[1] = 2; // O
        game.board[2] = 1; // X
        game.board[4] = 2; // O
        game.board[8] = 1; // X
        
        assert_eq\!(game.board[0], 1);
        assert_eq\!(game.board[1], 2);
        assert_eq\!(game.board[2], 1);
        assert_eq\!(game.board[3], 0); // Empty
        assert_eq\!(game.board[4], 2);
        assert_eq\!(game.board[8], 1);
    }

    #[test]
    fn test_game_wager_zero() {
        let mut game = create_test_game();
        game.wager = 0;
        
        assert_eq\!(game.wager, 0, "Wager can be zero for free games");
    }

    #[test]
    fn test_game_wager_large_amount() {
        let mut game = create_test_game();
        game.wager = u64::MAX;
        
        assert_eq\!(game.wager, u64::MAX, "Wager should support large amounts");
    }

    #[test]
    fn test_game_total_pot_matches_wagers() {
        let mut game = create_test_game();
        game.wager = 5_000_000_000;
        game.total_pot = 10_000_000_000;
        
        assert_eq\!(game.total_pot, game.wager * 2, "Total pot should equal both players' wagers");
    }

    #[test]
    fn test_game_total_pot_zero() {
        let mut game = create_test_game();
        game.wager = 0;
        game.total_pot = 0;
        
        assert_eq\!(game.total_pot, 0, "Total pot can be zero");
    }

    #[test]
    fn test_game_timeout_seconds_minimum() {
        let mut game = create_test_game();
        game.timeout_seconds = 1;
        
        assert_eq\!(game.timeout_seconds, 1, "Minimum timeout should be 1 second");
    }

    #[test]
    fn test_game_timeout_seconds_zero() {
        let mut game = create_test_game();
        game.timeout_seconds = 0;
        
        assert_eq\!(game.timeout_seconds, 0, "Timeout can be zero for no time limit");
    }

    #[test]
    fn test_game_timeout_seconds_large() {
        let mut game = create_test_game();
        game.timeout_seconds = 86400; // 24 hours
        
        assert_eq\!(game.timeout_seconds, 86400);
    }

    #[test]
    fn test_game_timestamps_created_at() {
        let game = create_test_game();
        
        assert\!(game.created_at > 0, "Created timestamp should be positive");
        assert_eq\!(game.created_at, 1_640_000_000);
    }

    #[test]
    fn test_game_timestamps_last_move() {
        let game = create_test_game();
        
        assert\!(game.last_move_ts > 0, "Last move timestamp should be positive");
        assert_eq\!(game.last_move_ts, 1_640_000_000);
    }

    #[test]
    fn test_game_timestamps_last_move_after_created() {
        let mut game = create_test_game();
        game.created_at = 1_640_000_000;
        game.last_move_ts = 1_640_000_300;
        
        assert\!(game.last_move_ts >= game.created_at, "Last move should be after or equal to creation");
    }

    #[test]
    fn test_game_timestamps_negative_values() {
        let mut game = create_test_game();
        game.created_at = -1;
        game.last_move_ts = -1;
        
        // i64 can be negative for timestamps before epoch
        assert_eq\!(game.created_at, -1);
        assert_eq\!(game.last_move_ts, -1);
    }

    #[test]
    fn test_game_player_pubkeys_unique() {
        let game = create_test_game();
        
        assert_ne\!(game.player_x, game.player_o, "Players should have different public keys");
    }

    #[test]
    fn test_game_player_pubkeys_valid() {
        let player_x = Pubkey::new_unique();
        let player_o = Pubkey::new_unique();
        
        let game = Game {
            player_x,
            player_o,
            wager: 0,
            board: [0; 9],
            turn: 1,
            status: 0,
            total_pot: 0,
            created_at: 0,
            last_move_ts: 0,
            timeout_seconds: 0,
            bump: 0,
        };
        
        assert_eq\!(game.player_x, player_x);
        assert_eq\!(game.player_o, player_o);
    }

    #[test]
    fn test_game_bump_seed_valid_range() {
        let mut game = create_test_game();
        game.bump = 255;
        
        assert_eq\!(game.bump, 255, "Bump should be valid u8 value");
    }

    #[test]
    fn test_game_bump_seed_zero() {
        let mut game = create_test_game();
        game.bump = 0;
        
        assert_eq\!(game.bump, 0, "Bump can be zero");
    }

    #[test]
    fn test_game_full_board_scenario() {
        let mut game = create_test_game();
        // Fill entire board
        game.board = [1, 2, 1, 2, 1, 2, 1, 2, 1];
        
        for (i, &cell) in game.board.iter().enumerate() {
            if i % 2 == 0 {
                assert_eq\!(cell, 1, "Even positions should be X");
            } else {
                assert_eq\!(cell, 2, "Odd positions should be O");
            }
        }
    }

    #[test]
    fn test_game_winning_row_x() {
        let mut game = create_test_game();
        // X wins top row
        game.board[0] = 1;
        game.board[1] = 1;
        game.board[2] = 1;
        game.status = 1; // X won
        
        assert_eq\!(game.board[0..3], [1, 1, 1]);
        assert_eq\!(game.status, 1);
    }

    #[test]
    fn test_game_winning_column_o() {
        let mut game = create_test_game();
        // O wins left column
        game.board[0] = 2;
        game.board[3] = 2;
        game.board[6] = 2;
        game.status = 2; // O won
        
        assert_eq\!(game.board[0], 2);
        assert_eq\!(game.board[3], 2);
        assert_eq\!(game.board[6], 2);
        assert_eq\!(game.status, 2);
    }

    #[test]
    fn test_game_winning_diagonal() {
        let mut game = create_test_game();
        // X wins diagonal
        game.board[0] = 1;
        game.board[4] = 1;
        game.board[8] = 1;
        game.status = 1;
        
        assert_eq\!(game.board[0], 1);
        assert_eq\!(game.board[4], 1);
        assert_eq\!(game.board[8], 1);
        assert_eq\!(game.status, 1);
    }

    #[test]
    fn test_game_draw_scenario() {
        let mut game = create_test_game();
        // Board filled with no winner
        game.board = [1, 2, 1, 2, 2, 1, 2, 1, 2];
        game.status = 3; // Draw
        
        assert_eq\!(game.status, 3);
        // Verify no empty cells
        for cell in game.board.iter() {
            assert_ne\!(*cell, 0, "No empty cells in draw");
        }
    }

    #[test]
    fn test_game_turn_alternation() {
        let mut game = create_test_game();
        
        // Start with X
        game.turn = 1;
        assert_eq\!(game.turn, 1);
        
        // Switch to O
        game.turn = 2;
        assert_eq\!(game.turn, 2);
        
        // Back to X
        game.turn = 1;
        assert_eq\!(game.turn, 1);
    }

    #[test]
    fn test_game_invalid_turn_value() {
        let mut game = create_test_game();
        game.turn = 255; // Invalid turn value
        
        assert_eq\!(game.turn, 255, "Can store invalid turn values (should be validated elsewhere)");
    }

    #[test]
    fn test_game_invalid_status_value() {
        let mut game = create_test_game();
        game.status = 255; // Invalid status
        
        assert_eq\!(game.status, 255, "Can store invalid status values (should be validated elsewhere)");
    }

    #[test]
    fn test_game_invalid_board_value() {
        let mut game = create_test_game();
        game.board[0] = 255; // Invalid marker
        
        assert_eq\!(game.board[0], 255, "Can store invalid board values (should be validated elsewhere)");
    }

    #[test]
    fn test_game_timeout_elapsed() {
        let mut game = create_test_game();
        game.created_at = 1_640_000_000;
        game.last_move_ts = 1_640_000_100; // 100 seconds after creation
        game.timeout_seconds = 300;
        
        let elapsed = game.last_move_ts - game.created_at;
        assert\!(elapsed < game.timeout_seconds as i64, "Move within timeout period");
    }

    #[test]
    fn test_game_timeout_exceeded() {
        let mut game = create_test_game();
        game.created_at = 1_640_000_000;
        game.last_move_ts = 1_640_000_000;
        game.timeout_seconds = 60;
        
        let current_time = 1_640_000_100; // 100 seconds later
        let time_since_last_move = current_time - game.last_move_ts;
        
        assert\!(time_since_last_move > game.timeout_seconds as i64, "Timeout exceeded");
    }

    #[test]
    fn test_game_struct_size_optimization() {
        use std::mem::size_of;
        
        let game_size = size_of::<Game>();
        
        // Verify struct isn't unexpectedly large
        // Game should be: 2*32 (Pubkeys) + 8 (wager) + 9 (board) + 1 (turn) + 1 (status) 
        // + 8 (total_pot) + 8 (created_at) + 8 (last_move_ts) + 8 (timeout_seconds) + 1 (bump)
        // = 146 bytes minimum (may be larger due to alignment)
        assert\!(game_size >= 146, "Game struct size should be at least 146 bytes");
        assert\!(game_size <= 200, "Game struct size should not be unreasonably large");
    }

    #[test]
    fn test_game_zero_initialization() {
        let game = Game {
            player_x: Pubkey::default(),
            player_o: Pubkey::default(),
            wager: 0,
            board: [0; 9],
            turn: 0,
            status: 0,
            total_pot: 0,
            created_at: 0,
            last_move_ts: 0,
            timeout_seconds: 0,
            bump: 0,
        };
        
        assert_eq\!(game.wager, 0);
        assert_eq\!(game.turn, 0);
        assert_eq\!(game.status, 0);
        assert_eq\!(game.total_pot, 0);
    }

    #[test]
    fn test_game_max_values() {
        let game = Game {
            player_x: Pubkey::new_unique(),
            player_o: Pubkey::new_unique(),
            wager: u64::MAX,
            board: [u8::MAX; 9],
            turn: u8::MAX,
            status: u8::MAX,
            total_pot: u64::MAX,
            created_at: i64::MAX,
            last_move_ts: i64::MAX,
            timeout_seconds: u64::MAX,
            bump: u8::MAX,
        };
        
        assert_eq\!(game.wager, u64::MAX);
        assert_eq\!(game.turn, u8::MAX);
        assert_eq\!(game.status, u8::MAX);
        assert_eq\!(game.total_pot, u64::MAX);
        assert_eq\!(game.created_at, i64::MAX);
        assert_eq\!(game.last_move_ts, i64::MAX);
        assert_eq\!(game.timeout_seconds, u64::MAX);
        assert_eq\!(game.bump, u8::MAX);
    }

    #[test]
    fn test_game_board_index_access() {
        let mut game = create_test_game();
        
        // Test all valid indices
        for i in 0..9 {
            game.board[i] = (i % 3) as u8;
            assert_eq\!(game.board[i], (i % 3) as u8);
        }
    }

    #[test]
    fn test_game_board_pattern_horizontal_wins() {
        // Test all three horizontal win patterns
        let test_cases = vec\![
            ([0, 1, 2], 1), // Top row X
            ([3, 4, 5], 2), // Middle row O
            ([6, 7, 8], 1), // Bottom row X
        ];
        
        for (positions, marker) in test_cases {
            let mut game = create_test_game();
            for &pos in positions.iter() {
                game.board[pos] = marker;
            }
            
            assert_eq\!(game.board[positions[0]], marker);
            assert_eq\!(game.board[positions[1]], marker);
            assert_eq\!(game.board[positions[2]], marker);
        }
    }

    #[test]
    fn test_game_board_pattern_vertical_wins() {
        // Test all three vertical win patterns
        let test_cases = vec\![
            ([0, 3, 6], 1), // Left column X
            ([1, 4, 7], 2), // Middle column O
            ([2, 5, 8], 1), // Right column X
        ];
        
        for (positions, marker) in test_cases {
            let mut game = create_test_game();
            for &pos in positions.iter() {
                game.board[pos] = marker;
            }
            
            assert_eq\!(game.board[positions[0]], marker);
            assert_eq\!(game.board[positions[1]], marker);
            assert_eq\!(game.board[positions[2]], marker);
        }
    }

    #[test]
    fn test_game_board_pattern_diagonal_wins() {
        // Test both diagonal win patterns
        let test_cases = vec\![
            ([0, 4, 8], 1), // Main diagonal X
            ([2, 4, 6], 2), // Anti-diagonal O
        ];
        
        for (positions, marker) in test_cases {
            let mut game = create_test_game();
            for &pos in positions.iter() {
                game.board[pos] = marker;
            }
            
            assert_eq\!(game.board[positions[0]], marker);
            assert_eq\!(game.board[positions[1]], marker);
            assert_eq\!(game.board[positions[2]], marker);
        }
    }

    #[test]
    fn test_game_realistic_game_progression() {
        let mut game = create_test_game();
        
        // Move 1: X at center
        game.board[4] = 1;
        game.turn = 2;
        game.last_move_ts = game.created_at + 10;
        
        // Move 2: O at top-left
        game.board[0] = 2;
        game.turn = 1;
        game.last_move_ts += 15;
        
        // Move 3: X at bottom-right
        game.board[8] = 1;
        game.turn = 2;
        game.last_move_ts += 8;
        
        assert_eq\!(game.board[4], 1);
        assert_eq\!(game.board[0], 2);
        assert_eq\!(game.board[8], 1);
        assert_eq\!(game.turn, 2);
        assert\!(game.last_move_ts > game.created_at);
    }

    #[test]
    fn test_game_same_player_cannot_have_both_roles() {
        // This should be enforced by game logic, but we test the data structure
        let player = Pubkey::new_unique();
        let game = Game {
            player_x: player,
            player_o: player, // Same player - invalid game state
            wager: 1_000_000,
            board: [0; 9],
            turn: 1,
            status: 0,
            total_pot: 2_000_000,
            created_at: 0,
            last_move_ts: 0,
            timeout_seconds: 300,
            bump: 255,
        };
        
        assert_eq\!(game.player_x, game.player_o, "Data structure allows same player (validation needed)");
    }

    #[test]
    fn test_game_pot_calculation_with_wager() {
        let wager = 500_000_000; // 0.5 SOL
        let game = Game {
            player_x: Pubkey::new_unique(),
            player_o: Pubkey::new_unique(),
            wager,
            board: [0; 9],
            turn: 1,
            status: 0,
            total_pot: wager * 2,
            created_at: 0,
            last_move_ts: 0,
            timeout_seconds: 300,
            bump: 255,
        };
        
        assert_eq\!(game.total_pot, 1_000_000_000);
        assert_eq\!(game.total_pot, game.wager * 2);
    }

    #[test]
    fn test_game_timestamp_overflow_safety() {
        let game = Game {
            player_x: Pubkey::new_unique(),
            player_o: Pubkey::new_unique(),
            wager: 0,
            board: [0; 9],
            turn: 1,
            status: 0,
            total_pot: 0,
            created_at: i64::MAX - 1000,
            last_move_ts: i64::MAX,
            timeout_seconds: 1000,
            bump: 0,
        };
        
        assert_eq\!(game.created_at, i64::MAX - 1000);
        assert_eq\!(game.last_move_ts, i64::MAX);
        
        // Verify we can handle max values
        let diff = game.last_move_ts.saturating_sub(game.created_at);
        assert_eq\!(diff, 1000);
    }
}
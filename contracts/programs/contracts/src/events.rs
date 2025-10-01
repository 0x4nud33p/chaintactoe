use anchor_lang::prelude::*;

#[event]
pub struct GameCreated {
    pub game: Pubkey,
    pub player_x: Pubkey,
    pub wager: u64,
    pub created_at: i64,
}

#[event]
pub struct GameJoined {
    pub game: Pubkey,
    pub player_o: Pubkey,
    pub joined_at: i64,
}
#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;

    // Helper function to create test Pubkey
    fn create_test_pubkey(seed: u8) -> Pubkey {
        Pubkey::new_from_array([seed; 32])
    }

    #[test]
    fn test_game_created_struct_construction() {
        let game_pubkey = create_test_pubkey(1);
        let player_x_pubkey = create_test_pubkey(2);
        let wager_amount = 1000u64;
        let timestamp = 1234567890i64;

        let game_created = GameCreated {
            game: game_pubkey,
            player_x: player_x_pubkey,
            wager: wager_amount,
            created_at: timestamp,
        };

        assert_eq\!(game_created.game, game_pubkey);
        assert_eq\!(game_created.player_x, player_x_pubkey);
        assert_eq\!(game_created.wager, wager_amount);
        assert_eq\!(game_created.created_at, timestamp);
    }

    #[test]
    fn test_game_created_with_zero_wager() {
        let game_pubkey = create_test_pubkey(1);
        let player_x_pubkey = create_test_pubkey(2);
        let zero_wager = 0u64;
        let timestamp = 1234567890i64;

        let game_created = GameCreated {
            game: game_pubkey,
            player_x: player_x_pubkey,
            wager: zero_wager,
            created_at: timestamp,
        };

        assert_eq\!(game_created.wager, 0);
        assert_eq\!(game_created.game, game_pubkey);
    }

    #[test]
    fn test_game_created_with_max_wager() {
        let game_pubkey = create_test_pubkey(1);
        let player_x_pubkey = create_test_pubkey(2);
        let max_wager = u64::MAX;
        let timestamp = 1234567890i64;

        let game_created = GameCreated {
            game: game_pubkey,
            player_x: player_x_pubkey,
            wager: max_wager,
            created_at: timestamp,
        };

        assert_eq\!(game_created.wager, u64::MAX);
    }

    #[test]
    fn test_game_created_with_negative_timestamp() {
        let game_pubkey = create_test_pubkey(1);
        let player_x_pubkey = create_test_pubkey(2);
        let wager_amount = 1000u64;
        let negative_timestamp = -1234567890i64;

        let game_created = GameCreated {
            game: game_pubkey,
            player_x: player_x_pubkey,
            wager: wager_amount,
            created_at: negative_timestamp,
        };

        assert_eq\!(game_created.created_at, negative_timestamp);
        assert\!(game_created.created_at < 0);
    }

    #[test]
    fn test_game_created_with_zero_timestamp() {
        let game_pubkey = create_test_pubkey(1);
        let player_x_pubkey = create_test_pubkey(2);
        let wager_amount = 1000u64;
        let zero_timestamp = 0i64;

        let game_created = GameCreated {
            game: game_pubkey,
            player_x: player_x_pubkey,
            wager: wager_amount,
            created_at: zero_timestamp,
        };

        assert_eq\!(game_created.created_at, 0);
    }

    #[test]
    fn test_game_created_with_max_timestamp() {
        let game_pubkey = create_test_pubkey(1);
        let player_x_pubkey = create_test_pubkey(2);
        let wager_amount = 1000u64;
        let max_timestamp = i64::MAX;

        let game_created = GameCreated {
            game: game_pubkey,
            player_x: player_x_pubkey,
            wager: wager_amount,
            created_at: max_timestamp,
        };

        assert_eq\!(game_created.created_at, i64::MAX);
    }

    #[test]
    fn test_game_created_same_player_and_game_pubkeys() {
        let same_pubkey = create_test_pubkey(1);
        let wager_amount = 1000u64;
        let timestamp = 1234567890i64;

        let game_created = GameCreated {
            game: same_pubkey,
            player_x: same_pubkey,
            wager: wager_amount,
            created_at: timestamp,
        };

        assert_eq\!(game_created.game, game_created.player_x);
    }

    #[test]
    fn test_game_created_default_pubkey() {
        let default_pubkey = Pubkey::default();
        let wager_amount = 500u64;
        let timestamp = 9876543210i64;

        let game_created = GameCreated {
            game: default_pubkey,
            player_x: default_pubkey,
            wager: wager_amount,
            created_at: timestamp,
        };

        assert_eq\!(game_created.game, Pubkey::default());
        assert_eq\!(game_created.player_x, Pubkey::default());
    }

    #[test]
    fn test_game_joined_struct_construction() {
        let game_pubkey = create_test_pubkey(1);
        let player_o_pubkey = create_test_pubkey(3);
        let timestamp = 1234567890i64;

        let game_joined = GameJoined {
            game: game_pubkey,
            player_o: player_o_pubkey,
            joined_at: timestamp,
        };

        assert_eq\!(game_joined.game, game_pubkey);
        assert_eq\!(game_joined.player_o, player_o_pubkey);
        assert_eq\!(game_joined.joined_at, timestamp);
    }

    #[test]
    fn test_game_joined_with_zero_timestamp() {
        let game_pubkey = create_test_pubkey(1);
        let player_o_pubkey = create_test_pubkey(3);
        let zero_timestamp = 0i64;

        let game_joined = GameJoined {
            game: game_pubkey,
            player_o: player_o_pubkey,
            joined_at: zero_timestamp,
        };

        assert_eq\!(game_joined.joined_at, 0);
    }

    #[test]
    fn test_game_joined_with_negative_timestamp() {
        let game_pubkey = create_test_pubkey(1);
        let player_o_pubkey = create_test_pubkey(3);
        let negative_timestamp = -9876543210i64;

        let game_joined = GameJoined {
            game: game_pubkey,
            player_o: player_o_pubkey,
            joined_at: negative_timestamp,
        };

        assert_eq\!(game_joined.joined_at, negative_timestamp);
        assert\!(game_joined.joined_at < 0);
    }

    #[test]
    fn test_game_joined_with_max_timestamp() {
        let game_pubkey = create_test_pubkey(1);
        let player_o_pubkey = create_test_pubkey(3);
        let max_timestamp = i64::MAX;

        let game_joined = GameJoined {
            game: game_pubkey,
            player_o: player_o_pubkey,
            joined_at: max_timestamp,
        };

        assert_eq\!(game_joined.joined_at, i64::MAX);
    }

    #[test]
    fn test_game_joined_with_min_timestamp() {
        let game_pubkey = create_test_pubkey(1);
        let player_o_pubkey = create_test_pubkey(3);
        let min_timestamp = i64::MIN;

        let game_joined = GameJoined {
            game: game_pubkey,
            player_o: player_o_pubkey,
            joined_at: min_timestamp,
        };

        assert_eq\!(game_joined.joined_at, i64::MIN);
    }

    #[test]
    fn test_game_joined_same_game_and_player_pubkeys() {
        let same_pubkey = create_test_pubkey(5);
        let timestamp = 1234567890i64;

        let game_joined = GameJoined {
            game: same_pubkey,
            player_o: same_pubkey,
            joined_at: timestamp,
        };

        assert_eq\!(game_joined.game, game_joined.player_o);
    }

    #[test]
    fn test_game_joined_default_pubkey() {
        let default_pubkey = Pubkey::default();
        let timestamp = 1111111111i64;

        let game_joined = GameJoined {
            game: default_pubkey,
            player_o: default_pubkey,
            joined_at: timestamp,
        };

        assert_eq\!(game_joined.game, Pubkey::default());
        assert_eq\!(game_joined.player_o, Pubkey::default());
    }

    #[test]
    fn test_multiple_game_created_events_independence() {
        let game1 = GameCreated {
            game: create_test_pubkey(1),
            player_x: create_test_pubkey(2),
            wager: 100u64,
            created_at: 1000i64,
        };

        let game2 = GameCreated {
            game: create_test_pubkey(3),
            player_x: create_test_pubkey(4),
            wager: 200u64,
            created_at: 2000i64,
        };

        assert_ne\!(game1.game, game2.game);
        assert_ne\!(game1.player_x, game2.player_x);
        assert_ne\!(game1.wager, game2.wager);
        assert_ne\!(game1.created_at, game2.created_at);
    }

    #[test]
    fn test_multiple_game_joined_events_independence() {
        let join1 = GameJoined {
            game: create_test_pubkey(1),
            player_o: create_test_pubkey(2),
            joined_at: 1000i64,
        };

        let join2 = GameJoined {
            game: create_test_pubkey(3),
            player_o: create_test_pubkey(4),
            joined_at: 2000i64,
        };

        assert_ne\!(join1.game, join2.game);
        assert_ne\!(join1.player_o, join2.player_o);
        assert_ne\!(join1.joined_at, join2.joined_at);
    }

    #[test]
    fn test_game_lifecycle_events_consistency() {
        let game_pubkey = create_test_pubkey(10);
        let player_x_pubkey = create_test_pubkey(20);
        let player_o_pubkey = create_test_pubkey(30);
        let create_time = 1000i64;
        let join_time = 2000i64;

        let created = GameCreated {
            game: game_pubkey,
            player_x: player_x_pubkey,
            wager: 500u64,
            created_at: create_time,
        };

        let joined = GameJoined {
            game: game_pubkey,
            player_o: player_o_pubkey,
            joined_at: join_time,
        };

        // Verify same game in both events
        assert_eq\!(created.game, joined.game);
        // Verify different players
        assert_ne\!(created.player_x, joined.player_o);
        // Verify join happened after creation
        assert\!(joined.joined_at > created.created_at);
    }

    #[test]
    fn test_game_created_large_wager_values() {
        let game_pubkey = create_test_pubkey(1);
        let player_x_pubkey = create_test_pubkey(2);
        let large_wager = 1_000_000_000_000u64; // 1 trillion lamports
        let timestamp = 1234567890i64;

        let game_created = GameCreated {
            game: game_pubkey,
            player_x: player_x_pubkey,
            wager: large_wager,
            created_at: timestamp,
        };

        assert_eq\!(game_created.wager, 1_000_000_000_000u64);
    }

    #[test]
    fn test_game_created_typical_wager_range() {
        let game_pubkey = create_test_pubkey(1);
        let player_x_pubkey = create_test_pubkey(2);
        let typical_wager = 1_000_000u64; // 0.001 SOL (1 million lamports)
        let timestamp = 1234567890i64;

        let game_created = GameCreated {
            game: game_pubkey,
            player_x: player_x_pubkey,
            wager: typical_wager,
            created_at: timestamp,
        };

        assert_eq\!(game_created.wager, 1_000_000u64);
        assert\!(game_created.wager > 0);
        assert\!(game_created.wager < u64::MAX);
    }

    #[test]
    fn test_game_joined_timestamp_progression() {
        let game_pubkey = create_test_pubkey(1);
        let player_o_pubkey = create_test_pubkey(3);
        let base_time = 1000000000i64;

        let join1 = GameJoined {
            game: game_pubkey,
            player_o: player_o_pubkey,
            joined_at: base_time,
        };

        let join2 = GameJoined {
            game: game_pubkey,
            player_o: player_o_pubkey,
            joined_at: base_time + 100,
        };

        assert\!(join2.joined_at > join1.joined_at);
        assert_eq\!(join2.joined_at - join1.joined_at, 100);
    }

    #[test]
    fn test_pubkey_array_construction() {
        let seed = 42u8;
        let pubkey1 = create_test_pubkey(seed);
        let pubkey2 = create_test_pubkey(seed);

        // Same seed should produce same pubkey
        assert_eq\!(pubkey1, pubkey2);

        let pubkey3 = create_test_pubkey(43);
        // Different seed should produce different pubkey
        assert_ne\!(pubkey1, pubkey3);
    }

    #[test]
    fn test_game_created_field_access() {
        let game_created = GameCreated {
            game: create_test_pubkey(1),
            player_x: create_test_pubkey(2),
            wager: 1000u64,
            created_at: 1234567890i64,
        };

        // Test individual field access
        let _ = game_created.game;
        let _ = game_created.player_x;
        let _ = game_created.wager;
        let _ = game_created.created_at;
    }

    #[test]
    fn test_game_joined_field_access() {
        let game_joined = GameJoined {
            game: create_test_pubkey(1),
            player_o: create_test_pubkey(3),
            joined_at: 1234567890i64,
        };

        // Test individual field access
        let _ = game_joined.game;
        let _ = game_joined.player_o;
        let _ = game_joined.joined_at;
    }

    #[test]
    fn test_game_created_clone_semantics() {
        let game_created = GameCreated {
            game: create_test_pubkey(1),
            player_x: create_test_pubkey(2),
            wager: 1000u64,
            created_at: 1234567890i64,
        };

        // Test that we can pass the struct around
        let verify_event = |event: &GameCreated| {
            assert_eq\!(event.wager, 1000u64);
        };

        verify_event(&game_created);
        // Original should still be accessible
        assert_eq\!(game_created.wager, 1000u64);
    }

    #[test]
    fn test_game_joined_clone_semantics() {
        let game_joined = GameJoined {
            game: create_test_pubkey(1),
            player_o: create_test_pubkey(3),
            joined_at: 1234567890i64,
        };

        // Test that we can pass the struct around
        let verify_event = |event: &GameJoined| {
            assert_eq\!(event.joined_at, 1234567890i64);
        };

        verify_event(&game_joined);
        // Original should still be accessible
        assert_eq\!(game_joined.joined_at, 1234567890i64);
    }

    #[test]
    fn test_timestamp_comparison_between_events() {
        let early_time = 1000i64;
        let late_time = 2000i64;

        let created = GameCreated {
            game: create_test_pubkey(1),
            player_x: create_test_pubkey(2),
            wager: 100u64,
            created_at: early_time,
        };

        let joined = GameJoined {
            game: create_test_pubkey(1),
            player_o: create_test_pubkey(3),
            joined_at: late_time,
        };

        assert\!(created.created_at < joined.joined_at);
        assert_eq\!(joined.joined_at - created.created_at, 1000);
    }

    #[test]
    fn test_wager_arithmetic() {
        let game_created = GameCreated {
            game: create_test_pubkey(1),
            player_x: create_test_pubkey(2),
            wager: 1000u64,
            created_at: 1234567890i64,
        };

        // Test wager can be used in arithmetic
        let doubled = game_created.wager * 2;
        assert_eq\!(doubled, 2000u64);

        let halved = game_created.wager / 2;
        assert_eq\!(halved, 500u64);
    }
}
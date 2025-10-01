
#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use std::mem::size_of;

    // ==================== Tests for validate_input_length ====================
    
    #[test]
    fn test_validate_input_length_empty_string() {
        // Test that empty strings are accepted
        let result = validate_input_length("");
        assert\!(result.is_ok(), "Empty string should be valid");
    }

    #[test]
    fn test_validate_input_length_single_char() {
        // Test minimum valid length
        let result = validate_input_length("a");
        assert\!(result.is_ok(), "Single character should be valid");
    }

    #[test]
    fn test_validate_input_length_max_valid() {
        // Test maximum valid length (280 characters)
        let max_valid = "a".repeat(280);
        let result = validate_input_length(&max_valid);
        assert\!(result.is_ok(), "280 characters should be valid");
    }

    #[test]
    fn test_validate_input_length_just_over_max() {
        // Test exactly one character over the limit
        let too_long = "a".repeat(281);
        let result = validate_input_length(&too_long);
        assert\!(result.is_err(), "281 characters should be invalid");
    }

    #[test]
    fn test_validate_input_length_way_over_max() {
        // Test significantly over the limit
        let way_too_long = "a".repeat(1000);
        let result = validate_input_length(&way_too_long);
        assert\!(result.is_err(), "1000 characters should be invalid");
    }

    #[test]
    fn test_validate_input_length_unicode_chars() {
        // Test with Unicode characters (emoji, special chars)
        let unicode_string = "Hello 🌍🚀✨";
        let result = validate_input_length(unicode_string);
        assert\!(result.is_ok(), "Unicode string within limit should be valid");
    }

    #[test]
    fn test_validate_input_length_unicode_at_limit() {
        // Test Unicode string at exactly the character limit
        let unicode_max = "🚀".repeat(280);
        let result = validate_input_length(&unicode_max);
        assert\!(result.is_ok(), "280 Unicode characters should be valid");
    }

    #[test]
    fn test_validate_input_length_unicode_over_limit() {
        // Test Unicode string over the limit
        let unicode_over = "🚀".repeat(281);
        let result = validate_input_length(&unicode_over);
        assert\!(result.is_err(), "281 Unicode characters should be invalid");
    }

    #[test]
    fn test_validate_input_length_whitespace_only() {
        // Test string with only whitespace
        let whitespace = "   ";
        let result = validate_input_length(whitespace);
        assert\!(result.is_ok(), "Whitespace within limit should be valid");
    }

    #[test]
    fn test_validate_input_length_newlines() {
        // Test string with newlines
        let with_newlines = "Line 1\nLine 2\nLine 3";
        let result = validate_input_length(with_newlines);
        assert\!(result.is_ok(), "String with newlines should be valid");
    }

    #[test]
    fn test_validate_input_length_special_chars() {
        // Test with special characters
        let special = "\!@#$%^&*()_+-=[]{}|;':\",./<>?";
        let result = validate_input_length(special);
        assert\!(result.is_ok(), "Special characters should be valid");
    }

    #[test]
    fn test_validate_input_length_mixed_content() {
        // Test with mixed alphanumeric and special characters
        let mixed = "Test123\!@# with spaces and números 456";
        let result = validate_input_length(mixed);
        assert\!(result.is_ok(), "Mixed content should be valid");
    }

    #[test]
    fn test_validate_input_length_boundary_279() {
        // Test one character below the limit
        let boundary = "a".repeat(279);
        let result = validate_input_length(&boundary);
        assert\!(result.is_ok(), "279 characters should be valid");
    }

    // ==================== Tests for calculate_space_required ====================

    #[test]
    fn test_calculate_space_required_zero_length() {
        // Test with zero-length string
        let space = calculate_space_required("");
        assert\!(space >= 8, "Should include discriminator");
        assert_eq\!(space, 8 + size_of::<u64>() + 0 + 4);
    }

    #[test]
    fn test_calculate_space_required_small_string() {
        // Test with small string
        let space = calculate_space_required("test");
        let expected = 8 + size_of::<u64>() + 4 + 4; // discriminator + timestamp + string length prefix + 4 bytes
        assert_eq\!(space, expected);
    }

    #[test]
    fn test_calculate_space_required_max_string() {
        // Test with maximum length string
        let max_string = "a".repeat(280);
        let space = calculate_space_required(&max_string);
        let expected = 8 + size_of::<u64>() + 280 + 4;
        assert_eq\!(space, expected);
    }

    #[test]
    fn test_calculate_space_required_unicode_string() {
        // Test with Unicode characters - bytes matter for storage
        let unicode = "Hello 🌍";
        let space = calculate_space_required(unicode);
        let byte_length = unicode.as_bytes().len();
        let expected = 8 + size_of::<u64>() + byte_length + 4;
        assert_eq\!(space, expected);
    }

    #[test]
    fn test_calculate_space_required_emoji_only() {
        // Test with only emoji (multi-byte characters)
        let emoji = "🚀🚀🚀";
        let space = calculate_space_required(emoji);
        let byte_length = emoji.as_bytes().len(); // Should be 12 bytes (4 per emoji)
        let expected = 8 + size_of::<u64>() + byte_length + 4;
        assert_eq\!(space, expected);
        assert\!(byte_length > 3, "Emoji should take multiple bytes");
    }

    #[test]
    fn test_calculate_space_required_multibyte_chars() {
        // Test with various multi-byte Unicode characters
        let multibyte = "日本語テスト";
        let space = calculate_space_required(multibyte);
        let byte_length = multibyte.as_bytes().len();
        let expected = 8 + size_of::<u64>() + byte_length + 4;
        assert_eq\!(space, expected);
    }

    #[test]
    fn test_calculate_space_required_single_char() {
        // Test with single character
        let single = "a";
        let space = calculate_space_required(single);
        let expected = 8 + size_of::<u64>() + 1 + 4;
        assert_eq\!(space, expected);
    }

    #[test]
    fn test_calculate_space_required_whitespace() {
        // Test with whitespace characters
        let whitespace = "   ";
        let space = calculate_space_required(whitespace);
        let expected = 8 + size_of::<u64>() + 3 + 4;
        assert_eq\!(space, expected);
    }

    #[test]
    fn test_calculate_space_required_newlines() {
        // Test with newlines
        let with_newlines = "a\nb\nc";
        let space = calculate_space_required(with_newlines);
        let expected = 8 + size_of::<u64>() + 5 + 4;
        assert_eq\!(space, expected);
    }

    #[test]
    fn test_calculate_space_required_special_chars() {
        // Test with special ASCII characters
        let special = "\!@#$%";
        let space = calculate_space_required(special);
        let expected = 8 + size_of::<u64>() + 5 + 4;
        assert_eq\!(space, expected);
    }

    #[test]
    fn test_calculate_space_required_consistency() {
        // Test that same string always returns same size
        let test_str = "consistency test";
        let space1 = calculate_space_required(test_str);
        let space2 = calculate_space_required(test_str);
        assert_eq\!(space1, space2, "Should be deterministic");
    }

    #[test]
    fn test_calculate_space_required_different_lengths() {
        // Test that different lengths produce different sizes
        let short = "hi";
        let long = "hello world";
        let space_short = calculate_space_required(short);
        let space_long = calculate_space_required(long);
        assert\!(space_long > space_short, "Longer strings should require more space");
        assert_eq\!(space_long - space_short, long.len() - short.len());
    }

    #[test]
    fn test_calculate_space_required_minimum_overhead() {
        // Test that minimum overhead is correct
        let space = calculate_space_required("");
        let min_overhead = 8 + size_of::<u64>() + 4; // discriminator + timestamp + string length prefix
        assert_eq\!(space, min_overhead);
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_validate_then_calculate_valid_input() {
        // Test workflow: validate input then calculate space
        let test_input = "Valid input string";
        let validation_result = validate_input_length(test_input);
        assert\!(validation_result.is_ok());
        
        let space = calculate_space_required(test_input);
        assert\!(space > 0);
        assert\!(space < 500); // Reasonable upper bound for this input
    }

    #[test]
    fn test_validate_then_calculate_max_valid() {
        // Test workflow with maximum valid input
        let max_input = "x".repeat(280);
        let validation_result = validate_input_length(&max_input);
        assert\!(validation_result.is_ok());
        
        let space = calculate_space_required(&max_input);
        let expected = 8 + size_of::<u64>() + 280 + 4;
        assert_eq\!(space, expected);
    }

    #[test]
    fn test_validate_fails_before_calculate() {
        // Test that validation catches invalid input before space calculation
        let invalid_input = "a".repeat(300);
        let validation_result = validate_input_length(&invalid_input);
        assert\!(validation_result.is_err());
        
        // Space calculation should still work (even though validation failed)
        let space = calculate_space_required(&invalid_input);
        assert\!(space > 0);
    }

    #[test]
    fn test_space_calculation_with_various_valid_inputs() {
        // Test multiple valid inputs
        let inputs = vec\![
            "",
            "a",
            "test",
            "Hello, World\!",
            "This is a longer test string with multiple words.",
            &"x".repeat(280),
        ];
        
        for input in inputs {
            assert\!(validate_input_length(input).is_ok());
            let space = calculate_space_required(input);
            assert\!(space >= 8 + size_of::<u64>() + 4);
        }
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_null_byte_in_string() {
        // Test handling of null bytes within string
        let with_null = "test\0string";
        let validation = validate_input_length(with_null);
        assert\!(validation.is_ok());
        
        let space = calculate_space_required(with_null);
        assert_eq\!(space, 8 + size_of::<u64>() + 11 + 4);
    }

    #[test]
    fn test_continuous_unicode() {
        // Test long string of continuous Unicode
        let unicode_string = "你好世界".repeat(50); // 200 characters
        let validation = validate_input_length(&unicode_string);
        assert\!(validation.is_ok());
        
        let space = calculate_space_required(&unicode_string);
        let byte_len = unicode_string.as_bytes().len();
        assert_eq\!(space, 8 + size_of::<u64>() + byte_len + 4);
    }

    #[test]
    fn test_mixed_ascii_unicode() {
        // Test mixing ASCII and Unicode characters
        let mixed = "Hello こんにちは 🌍 World";
        let validation = validate_input_length(mixed);
        assert\!(validation.is_ok());
        
        let space = calculate_space_required(mixed);
        let byte_len = mixed.as_bytes().len();
        assert_eq\!(space, 8 + size_of::<u64>() + byte_len + 4);
    }

    #[test]
    fn test_all_special_characters() {
        // Test with comprehensive set of special characters
        let special = "\!@#$%^&*()_+-=[]{}\\|;':\",./<>?`~";
        let validation = validate_input_length(special);
        assert\!(validation.is_ok());
        
        let space = calculate_space_required(special);
        assert\!(space > 0);
    }

    #[test]
    fn test_tabs_and_carriage_returns() {
        // Test with tabs and carriage returns
        let whitespace_variants = "tab\there\r\nand\tthere";
        let validation = validate_input_length(whitespace_variants);
        assert\!(validation.is_ok());
        
        let space = calculate_space_required(whitespace_variants);
        assert_eq\!(space, 8 + size_of::<u64>() + whitespace_variants.len() + 4);
    }

    // ==================== Property-Based Tests ====================

    #[test]
    fn test_space_increases_with_length() {
        // Property: space should increase linearly with ASCII string length
        let sizes: Vec<usize> = (0..10).map(|i| {
            let s = "a".repeat(i * 10);
            calculate_space_required(&s)
        }).collect();
        
        for i in 1..sizes.len() {
            assert\!(sizes[i] > sizes[i-1], "Space should increase with length");
        }
    }

    #[test]
    fn test_validation_boundary_properties() {
        // Property: strings at boundary should have specific behavior
        assert\!(validate_input_length(&"a".repeat(280)).is_ok());
        assert\!(validate_input_length(&"a".repeat(281)).is_err());
        
        // Test boundary around the limit
        for len in 275..285 {
            let s = "x".repeat(len);
            let result = validate_input_length(&s);
            if len <= 280 {
                assert\!(result.is_ok(), "Length {} should be valid", len);
            } else {
                assert\!(result.is_err(), "Length {} should be invalid", len);
            }
        }
    }

    #[test]
    fn test_space_calculation_overhead_constant() {
        // Property: overhead should be constant regardless of content
        let overhead1 = calculate_space_required("") - 0;
        let overhead2 = calculate_space_required("test") - 4;
        let overhead3 = calculate_space_required("longer test") - 11;
        
        assert_eq\!(overhead1, overhead2);
        assert_eq\!(overhead2, overhead3);
    }
}

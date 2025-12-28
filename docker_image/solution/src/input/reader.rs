pub fn get_player_id(input: &str) -> Option<u8> {
    let clean_input = input.trim();

    if !clean_input.contains("$$$ exec p") {
        return None;
    }

    // Get the prefix
    let prefix = "$$$ exec p";
    let after = &clean_input[prefix.len()..];


    // Extract first character (should be '1' or '2')
    if let Some(first_char) = after.chars().next() {
        if first_char == '1' {
            return Some(1);
        } else if first_char == '2' {
            return Some(2);
        }
    }

    None
}
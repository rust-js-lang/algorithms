use std::collections::VecDeque;

/// The direction the zigzag goes along the rails.
const UP: isize = -1;
const DOWN: isize = 1;

/// The direction the next character goes in, from `current_rail`: down
/// from the top, up from the bottom, else on as it goes.
fn get_next_direction(rail_count: usize, current_rail: usize, direction: isize) -> isize {
    if current_rail == 0 {
        DOWN
    } else if current_rail == rail_count - 1 {
        UP
    } else {
        direction
    }
}

/// The rail each of `length` characters is on, zigzagging down and up
/// `rail_count` rails.
fn zigzag(length: usize, rail_count: usize) -> Vec<usize> {
    let mut rails = Vec::with_capacity(length);
    let mut current_rail = 0;
    let mut direction = DOWN;
    for _ in 0..length {
        rails.push(current_rail);
        direction = get_next_direction(rail_count, current_rail, direction);
        current_rail = current_rail.saturating_add_signed(direction);
    }
    rails
}

/// `string`, its characters written down and up `rail_count` rails, read a
/// rail at a time.
pub fn encode_rail_fence_cipher(string: &str, rail_count: usize) -> String {
    let chars: Vec<char> = string.chars().collect();
    let mut fence = vec![Vec::new(); rail_count];
    for (char, rail) in chars.iter().zip(zigzag(chars.len(), rail_count)) {
        fence[rail].push(*char);
    }
    fence.concat().into_iter().collect()
}

/// The string whose rail fence cipher of `rail_count` rails `string` is.
pub fn decode_rail_fence_cipher(string: &str, rail_count: usize) -> String {
    let chars: Vec<char> = string.chars().collect();
    let rails = zigzag(chars.len(), rail_count);

    // Each rail's characters, as many as the zigzag puts on it, in order.
    let mut fence: Vec<VecDeque<char>> = vec![VecDeque::new(); rail_count];
    let mut rest = chars.into_iter();
    for (rail, chars) in fence.iter_mut().enumerate() {
        let count = rails.iter().filter(|&&r| r == rail).count();
        chars.extend(rest.by_ref().take(count));
    }

    // Read along the zigzag.
    rails
        .iter()
        .filter_map(|&rail| fence[rail].pop_front())
        .collect()
}

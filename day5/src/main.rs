use std::{collections::HashMap, fs, print};

fn main() -> Result<(), std::io::Error> {
    // comment post solution - much easier solution to treat as binary ROW[XXXXXXX]COL[XXX] which in binary
    // translates directly to seat id (because ROW starts at pos 4 [_000], has to be multiplied by 8)
    let input = fs::read_to_string("input.txt")?;
    p2_fast(&input);
    return Ok(());
    let seats: Vec<(usize, usize)> = input.lines().map(|x| get_seat(x).unwrap()).collect();
    let max_seat = seats.iter().max_by_key(|(row, col)| (row * 8) + col).unwrap();
    let max_id = (max_seat.0 * 8) + max_seat.1;

    let mut map = HashMap::new();
    seats.iter().for_each(|(row, col)| {
        map.insert((*row, *col), (row * 8) + col);
    });

    for r in 1..127usize {
        for c in 0..8usize {
            if !map.contains_key(&(r, c)) {
                let current_id = (r * 8) + c;
                let has_lower = has_seat_with_id(&map, current_id - 1);
                let has_upper = has_seat_with_id(&map, current_id + 1);
                match (has_lower, has_upper) {
                    (Some(true), Some(true)) => println!("Might have at {}/{}, id {}", r, c, current_id),
                    _ => {},
                };
                // println!("Has empty at {}/{}, id {}", r, c, current_id);
            }
        }
    }
    println!("Max - {}", max_id);
    Ok(())
}

fn has_seat_with_id(map: &HashMap<(usize, usize), usize>, id: usize) -> Option<bool> {
    Some(map.values().any(|x| x == &id))
}

fn get_seat(line: &str) -> Option<(usize, usize)> {
    let line_chars: Vec<char> = line.chars().collect();
    Some((
            get_seat_pos(&line_chars[..7], 0, 127).unwrap(),
            get_seat_pos(&line_chars[7..], 0, 7).unwrap()
    ))
}

fn get_seat_pos(moves: &[char], lower: usize, upper: usize) -> Option<usize> {
    if let Some(current_move) = moves.first() {
        let half = (upper as f64 - lower as f64) / 2.0;
        let pos = match current_move {
            'F' => get_seat_pos(&moves[1..], lower, lower + half.floor() as usize),
            'B' => get_seat_pos(&moves[1..], lower + half.ceil() as usize, upper),

            // Left - Right
            'L' => get_seat_pos(&moves[1..], lower, lower + half.floor() as usize),
            'R' => get_seat_pos(&moves[1..], lower + half.ceil() as usize, upper),
            _ => panic!("Unexpected side"),
        };

        return pos;
    }

    Some(lower)
}

fn p2_fast(input: &String) {
    let mut transformed: Vec<u64> = input.lines().map(|x| get_id_from_str(x).unwrap()).collect();
    transformed.sort();

    for (idx, id) in transformed.iter().enumerate() {
        if let Some(next_id) = transformed.get(idx + 1) {
            if id + 2 == *next_id { // If there is next element and next element id is +2, we have a gap
                let s: String = format!("{:10b}", id+1).chars().collect();

                println!("Might be {}/{}", s, id+1);
            }
        }
    }
}

fn get_id_from_str(input: &str) -> Option<u64> {
    // Remap to binary string
    let transformed: String = input.chars().map(|c| if c == 'B' || c == 'R' { '1' } else { '0' }).collect();

    // Convert to u64
    Some(u64::from_str_radix(&transformed, 2).unwrap())
}

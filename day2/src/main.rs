use std::fs;

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    let input_split: Vec<&str> = input.lines().collect();
    let valid_count: usize = input_split.iter().filter(|x| is_valid_password(x).unwrap()).count();
    let valid_count_2: usize = input_split.iter().filter(|x| is_valid_password_2(x).unwrap()).count();

    println!("Valid 1 - {}", valid_count);
    println!("Valid 2 - {}", valid_count_2);
    Ok(())
}

fn is_valid_password(password_line: &str) -> Option<bool> {
    if password_line.len() <= 0 {
        return Some(false)
    }

    let mut line_iter = password_line.split_whitespace();
    let from_to_values: Vec<i32> = line_iter.next()?.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
    if let [from, to] = &from_to_values[..] {
        let letter = line_iter.next()?.replace(":", "").chars().next()?; // Should only be single char
        let password = line_iter.next()?;
        let count_of_letter = password.chars().filter(|x| x == &letter).count() as i32;

        return Some(&count_of_letter >= from && &count_of_letter <= to)
    }

    Some(false)
}

fn is_valid_password_2(password_line: &str) -> Option<bool> {
    if password_line.len() <= 0 {
        return Some(false)
    }

    let mut line_iter = password_line.split_whitespace();
    let from_to_values: Vec<usize> = line_iter.next()?.split("-").map(|x| x.parse::<usize>().unwrap()).collect();
    if let [p1, p2] = &from_to_values[..] {
        let letter = line_iter.next()?.replace(":", "").chars().next()?; // Should only be single char
        let password: Vec<char> = line_iter.next()?.chars().collect();

        return Some((password[p1-1] == letter) ^ (password[p2-1] == letter))
    }

    Some(false)
}


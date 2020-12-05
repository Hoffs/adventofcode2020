use std::fs;

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    let input_line: Vec<Vec<&str>> = input.split("\n\n").map(|x| x.split_whitespace().collect()).collect();
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // , "cid"];

    let valid_count: usize = input_line.iter().filter(|x| is_valid_passport(x, &required_fields).unwrap()).count();
    let valid_count_2: usize = input_line.iter().filter(|x| is_valid_passport_values(x, &required_fields).unwrap()).count();
    println!("Valid - {}", valid_count);
    println!("Valid 2 - {}", valid_count_2);
    Ok(())
}

fn is_valid_passport(passport_entry: &Vec<&str>, required_fields: &Vec<&str>) -> Option<bool> {
    return Some(
        required_fields
            .iter()
            .all(|x| passport_entry
                .iter()
                .any(|y|
                    y.contains(&format!("{}:", x))
                    )
                )
            );
}

fn is_valid_passport_values(passport_entry: &Vec<&str>, required_fields: &Vec<&str>) -> Option<bool> {
    return Some(
        required_fields
            .iter()
            .all(|x| passport_entry
                .iter()
                .any(|y|
                    y.contains(&format!("{}:", x))
                    )
                )
        && passport_entry.iter().all(|x| validate_field(x).unwrap())
    );
}

fn validate_field(field: &str) -> Option<bool> {
    let kv: Vec<&str> = field.splitn(2, ":").collect();
    if let [key, val] = &kv[..] {
        match *key {
            "byr" => {
                let year = val.parse::<usize>().ok().unwrap_or(0);
                if year >= 1920 && year <= 2002 {
                    return Some(true)
                }

                //
            },
            "iyr" => {
                let year = val.parse::<usize>().ok().unwrap_or(0);
                if year >= 2010 && year <= 2020 {
                    return Some(true)
                }
            },
            "eyr" => {
                let year = val.parse::<usize>().ok().unwrap_or(0);
                if year >= 2020 && year <= 2030 {
                    return Some(true)
                }
            },
            "hgt" => {
                let is_cm = val.contains("cm");
                let is_in = val.contains("in");
                let height = val.replace("cm", "").replace("in", "").parse::<usize>().ok().unwrap_or(0);
                if (is_cm && height >= 150 && height <= 193) || (is_in && height >= 59 && height <= 76) {
                    return Some(true)
                }
            },
            "hcl" => {
                let chars: Vec<char> = val.chars().collect();
                if val.starts_with("#") /* && val.len() == 7 */ && chars[1..].iter().all(|x| (x >= &'0' && x <= &'9') || (x >= &'a' && x <= &'f')) {
                    return Some(true)
                }
            },
            "ecl" => {
                let valid_vals = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                if valid_vals.contains(val) {
                    return Some(true)
                }
            },
            "pid" => {
                let chars: Vec<char> = val.chars().collect();
                if val.len() == 9 && chars.iter().all(|x| x >= &'0' && x <= &'9') {
                    return Some(true)
                }
            }
            "cid" => { return Some(true) },
            _ => panic!("Unexpected key")
        }
    }

    Some(false)
}

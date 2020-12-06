use std::{collections::HashSet, collections::HashMap, fs};

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input.txt")?;

    let group_inputs: Vec<Vec<_>> = input.split("\n\n").map(|x| x.lines().collect()).collect();

    let total_answered: usize = group_inputs.iter().map(|x| get_group_count(x).unwrap()).sum();

    println!("Total - {}", total_answered);

    let total_answered_p2: usize = group_inputs.iter().map(|x| get_group_count_p2(x).unwrap()).sum();

    println!("Total p2 - {}", total_answered_p2);

    Ok(())
}

fn get_group_count(input: &Vec<&str>) -> Option<usize> {
    let mut answered = HashSet::new();

    for line in input {
        line.chars().for_each(|c| { answered.insert(c); });
    }

    Some(answered.len())
}

fn get_group_count_p2(input: &Vec<&str>) -> Option<usize> {
    // Could use intersect a set for each person, but this seemed "faster".
    // Single loop for setting letters+count and another loop for filtering that were answered enough times.
    let mut answered: HashMap<char, usize> = HashMap::new();

    for line in input {
        line.chars().for_each(|c| { *answered.entry(c).or_insert(0) += 1; });
    }

    let required = input.len();
    Some(answered.iter().filter(|(_, v)| *v == &required).count())
}

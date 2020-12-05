use std::fs;
use itertools::Itertools;

fn main() -> Result<(), std::io::Error> {
    let combination_count: usize = 3;
    let input: Vec<i32> = fs::read_to_string("input.txt")?.split_whitespace().map(|x| x.parse().unwrap() ).collect();
    let mut input_combos = input.into_iter().combinations(combination_count);
    let winning_combo = input_combos.find(|x: &Vec<i32>| x.iter().sum::<i32>() == 2020);

    if let Some(x) = winning_combo {
        let result = x.into_iter().fold(1, |acc, val| acc * val);
        println!("{}", result)
    }

    Ok(())
}

use std::fs;

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("input.txt")?;
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let moves: Vec<(usize, usize)> = vec![(3, 1), (1, 1), (5, 1), (7, 1), (1, 2)];

    let total = moves.iter().map(|(right, down)| traverse_map(&map, *right, *down).unwrap()).fold(1, |acc, x| acc * x);

    println!("Trees {}", total);
    Ok(())
}

fn traverse_map(map: &Vec<Vec<char>>, move_right: usize, move_down: usize) -> Option<usize> {
    let map_width = map.get(0).unwrap().len();
    let mut x = move_right;
    let mut y = move_down;
    let mut counter = 0;

    println!("Len {}", map.len());
    while y < map.len() {
        x %= map_width;

        // println!("Checking {}/{}", x, y);
        if map.get(y).unwrap().get(x).unwrap() == &'#' {
            counter += 1;
        }

        x += move_right;
        y += move_down;
    }

    println!("Counter {}", counter);
    Some(counter)
}

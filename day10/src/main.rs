use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};

fn read_file_to_string(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find_paths(
    map: &Vec<Vec<char>>,
    start_location: (usize, usize),
) -> io::Result<Vec<(usize, usize)>> {
    let (row, column) = start_location;
    let current_value = map[row][column];
    let mut outcome = Vec::new();
    let mut path_found = false;

    if current_value == '9' {
        outcome.push(start_location);
        return Ok(outcome);
    }
    if row > 0 && map[row - 1][column] == (current_value as u8 + 1) as char {
        let mut path = find_paths(map, (row - 1, column))?;
        outcome.append(&mut path);
        path_found = true;
    }
    if row < map.len() - 1 && map[row + 1][column] == (current_value as u8 + 1) as char {
        let mut path = find_paths(map, (row + 1, column))?;
        outcome.append(&mut path);
        path_found = true;
    }
    if column > 0 && map[row][column - 1] == (current_value as u8 + 1) as char {
        let mut path = find_paths(map, (row, column - 1))?;
        outcome.append(&mut path);
        path_found = true;
    }
    if column < map[row].len() - 1 && map[row][column + 1] == (current_value as u8 + 1) as char {
        let mut path = find_paths(map, (row, column + 1))?;
        outcome.append(&mut path);
        path_found = true;
    }

    if path_found {
        Ok(outcome)
    } else {
        Ok(Vec::new())
    }
}

fn remove_duplicates(vec: &mut Vec<(usize, usize)>) {
    let mut unique_elements = HashSet::new();
    vec.retain(|item| unique_elements.insert(item.clone()));
}

fn main() -> io::Result<()> {
    let file_path = "Day10_map.txt";
    let contents = read_file_to_string(file_path)?;
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut trailhead_score = 0;
    let mut trail_score = 0;
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if map[row][column] == '0' {
                // trail_score += find_paths(&map, (row, column))?;
                let mut locations = find_paths(&map, (row, column))?;
                trail_score += locations.len();
                remove_duplicates(&mut locations);
                trailhead_score += locations.len();
            }
        }
    }

    println!("Trailheads score: {}", trailhead_score);
    println!("Trail score: {}", trail_score);
    Ok(())
}

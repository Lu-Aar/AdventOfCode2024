use std::fs::File;
use std::io::{self, Read};

const NUMB_OF_BLINKS: u8 = 75;

fn read_file_to_string(file_path: &str) -> io::Result<Vec<String>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let stones: Vec<String> = contents.split_whitespace().map(String::from).collect();

    Ok(stones)
}

fn blink(stones: Vec<String>) -> io::Result<Vec<String>> {
    let mut blinked_stones = Vec::new();
    for stone in stones {
        if stone.parse::<u128>().unwrap() == 0 {
            blinked_stones.push("1".to_string());
        } else if stone.len() % 2 == 0 {
            let split_length = stone.len() / 2;
            blinked_stones.push(stone[..split_length].parse::<u128>().unwrap().to_string());
            blinked_stones.push(stone[split_length..].parse::<u128>().unwrap().to_string());
        } else {
            blinked_stones.push((stone.parse::<u128>().unwrap() * 2024).to_string());
        }
    }
    Ok(blinked_stones)
}

// fn blink(stones: &mut Vec<String>) -> io::Result<()> {
//     let mut ii = stones.len();
//     while ii > 0 {
//         ii -= 1;
//         if stones[ii].parse::<u128>().unwrap() == 0 {
//             stones[ii] = "1".to_string();
//         } else if stones[ii].len() % 2 == 0 {
//             let split_length = stones[ii].len() / 2;

//             if ii == stones.len() - 1 {
//                 stones.push(
//                     stones[ii][split_length..]
//                         .parse::<u128>()
//                         .unwrap()
//                         .to_string(),
//                 );
//             } else {
//                 stones.insert(
//                     ii + 1,
//                     stones[ii][split_length..]
//                         .parse::<u128>()
//                         .unwrap()
//                         .to_string(),
//                 );
//             }

//             stones[ii] = stones[ii][..split_length]
//                 .parse::<u128>()
//                 .unwrap()
//                 .to_string();
//         } else {
//             stones[ii] = (stones[ii].parse::<u128>().unwrap() * 2024).to_string();
//         }
//     }
//     Ok(())
// }

// fn blink(stones: &mut Vec<String>) -> io::Result<()> {
//     let mut ii = stones.len();
//     while ii > 0 {
//         ii -= 1;
//         let stone = &mut stones[ii]; // Borrow mutably once

//         match stone.parse::<u128>() {
//             Ok(0) => *stone = "1".to_string(), // Dereference and assign directly
//             Ok(_) if stone.len() % 2 == 0 => {
//                 let split_length = stone.len() / 2;
//                 let (left, right_str) = stone.split_at(split_length);

//                 match right_str.parse::<u128>() {
//                     Ok(right_num) => {
//                         *stone = left.to_string(); // Assign left directly
//                         stones.insert(ii + 1, right_num.to_string()); // Insert right
//                     }
//                     Err(_) => continue, // Handle parsing errors
//                 }
//             }
//             Ok(num) => *stone = (num * 2024).to_string(),
//             Err(_) => {
//                 // Handle parsing errors. You might want to log an error,
//                 // skip the stone, or return an Err. For this example, we skip.
//                 continue;
//             }
//         }
//     }
//     Ok(())
// }

fn main() -> io::Result<()> {
    let mut stones = read_file_to_string("Day11_stones.txt")?;

    for ii in 0..NUMB_OF_BLINKS {
        println!("{}:{}", ii, stones.len());
        stones = blink(stones)?;
    }

    println!("{}", stones.len());

    Ok(())
}

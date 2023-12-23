// struct Game {
//     red: u32,
//     green: u32,
//     blue: u32,
// }

use std::fs;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

// Determine which games would have been possible if the bag had been loaded with only:
// 12 red cubes, 13 green cubes, and 14 blue cubes.
// What is the sum of the IDs of those games?
fn process(input: &str) -> u32 {
    let mut res_sum = 0;
    for line in input.lines() {
        let game: Vec<&str> = line.split(':').collect();
        let game_id = game[0].replace("Game ", "").parse::<u32>().unwrap();
        let parties: Vec<&str> = game[1].split(';').collect();
        let mut is_game_valid = true;

        for party in parties {
            let cubes: Vec<&str> = party.split(',').collect();
            for cube in cubes {
                let sections: Vec<&str> = cube.split(' ').collect();
                let count = sections[1].parse::<u32>().unwrap();
                let color = sections[2];
                // let mut valid_cube_comb = true;
                match color {
                    "red" => {
                        if count > MAX_RED {
                            is_game_valid = false;
                            break;
                        }
                    }
                    "green" => {
                        if count > MAX_GREEN {
                            is_game_valid = false;
                            break;
                        }
                    }
                    "blue" => {
                        if count > MAX_BLUE {
                            is_game_valid = false;
                            break;
                        }
                    }
                    _ => panic!("Encountered invalid path!"),
                }
                if !is_game_valid {
                    break;
                }
            }
            if !is_game_valid {
                break;
            }
        }
        if is_game_valid {
            res_sum += game_id;
        }
    }

    res_sum
}
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Not a file");
    let sum = process(&contents);
    println!("{sum}");
}

mod test {
    #[test]
    fn test_input() {
        use super::process;
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(process(input), 8);
    }
}

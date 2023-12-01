use std::fs;

use regex::Regex;

// const digit_dict: HashMap<&str, u32> = HashMap::from([("one", 1)]);
/// In day one we need to extract the first and second digits from a line
fn process(input: &str) -> u32 {
    const DIGIT_DICT: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut out_str = String::new();
    let mut sum = 0;

    for line in input.lines() {
        // index of char and char itself
        let mut digits: Vec<(usize, char)> = vec![];

        // FIXME: (aver) fix not finding duplicate occurences
        for (digit_int, digit_str) in DIGIT_DICT.into_iter().enumerate() {
            let re = Regex::new(digit_str).unwrap();
            for mtch in re.find_iter(line) {
                digits.push((
                    mtch.start(),
                    char::from_digit(digit_int as u32, 10).unwrap(),
                ));
            }
            // match line.find(digit_str) {
            //     Some(i) => digits.push((i, char::from_digit(digit_int as u32, 10).unwrap())),
            //     None => (),
            // }
        }

        // println!("{:?}", digits);
        // let digits: Vec<(usize, char)> = line
        digits.extend(
            line.chars()
                .enumerate()
                .filter(|(idx, char)| char.is_digit(10)),
        );
        // println!("{:?}", digits);
        digits.sort_by_key(|k| k.0);
        println!("{:?}", digits);

        if digits.len() == 0 {
            panic!("Line does not contain digits")
        }

        out_str.push(digits.first().unwrap().1.to_owned());
        // if digits.len() > 1 {
        out_str.push(digits.last().unwrap().1.to_owned());
        // }
        println!("{out_str}");

        sum += out_str.parse::<u32>().unwrap();
        out_str.clear();
    }

    sum
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Not a file");
    let sum = process(&contents);
    println!("{sum}");
}

mod test {
    #[test]
    fn test_input_2() {
        use super::*;
        let default_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(process(default_input), 281);
    }
}

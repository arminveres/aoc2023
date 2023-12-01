use std::fs;
/// In day one we need to extract the first and second digits from a line
fn process(input: &str) -> u32 {
    let mut out_str = String::new();
    let mut sum = 0;

    for line in input.lines() {
        // let digits: Vec<char> = line.chars().filter(|char| char.is_digit(10)).collect();
        // if digits.len() == 0 {
        //     panic!("Line does not contain digits")
        // }
        // out_str.push(digits.first().unwrap().to_owned());
        // out_str.push(digits.last().unwrap().to_owned());

        // NOTE: (aver) could add a reverse iterator and use find on it, instead of filtering the
        // whole line
        let digits_one = line.chars().find(|char| char.is_digit(10)).unwrap();
        let digits_two = line.chars().rev().find(|char| char.is_digit(10)).unwrap();
        out_str.push(digits_one);
        out_str.push(digits_two);

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
    fn test_input_1() {
        use super::*;
        let default_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process(default_input), 142);
    }
}

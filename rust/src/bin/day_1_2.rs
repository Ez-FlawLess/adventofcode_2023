use std::fs;

const DIGITS_TEXT: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {

    let data = fs::read_to_string("..\\inputs\\day_1.txt")
        .expect("couldn't read file to string");

    let answer = data.lines()
        .map(|line| {

            let mut nums = (0_u8, None);

            for index in 0..line.len() {
                let line = &line[index..];
                if let Some(digit_index) = DIGITS_TEXT.iter().position(|&digit_text| {
                    line.starts_with(digit_text)
                }) {
                    let digit = digit_index + 1;
                    nums.0 = digit as u8;
                    break;
                } else {
                    let letter = line.as_bytes()[0];
                    if letter.is_ascii_digit() {
                        nums.0 = letter - 48;
                        break;
                    }
                }
            }

            for index in 0..line.len() {
                let line = &line[index..];
                if let Some(digit_index) = DIGITS_TEXT.iter().position(|&digit_text| {
                    line.starts_with(digit_text)
                }) {
                    let digit = digit_index + 1;
                    nums.1 = Some(digit as u8);
                } else {
                    let letter = line.as_bytes()[0];
                    if letter.is_ascii_digit() {
                        nums.1 = Some(letter  - 48);
                    }
                }
            }

            format!("{}{}", nums.0, nums.1.unwrap_or(nums.0)).parse::<u32>().unwrap()
        })
        .sum::<u32>();

    print!("{answer}");

}
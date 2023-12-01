use std::fs;

const DIGITS_TEXT: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {

    let data = fs::read_to_string("..\\inputs\\day_1.txt")
        .expect("couldn't read file to string");

    let answer = calc_answer(&data);

    print!("{answer}");

    assert_eq!(answer, 54885);

}

fn get_digit_text(text: &str) -> Option<u8> {
    DIGITS_TEXT.iter()
        .position(|&digit_text| {
            text.starts_with(digit_text)
        })
        .map(|position| (position + 1) as u8)
}

fn calc_answer(data: &str) -> u32 {
    data.lines()
        .map(|line| {

            let mut nums = (0_u8, None);

            let mut num_1_index = 0;

            for index in 0..line.len() {
                let line = &line[index..];
                if let Some(digit) = get_digit_text(line) {
                    nums.0 = digit;
                    break;
                } else {
                    let letter = line.as_bytes()[0];
                    if letter.is_ascii_digit() {
                        nums.0 = letter - 48;
                        num_1_index = index;
                        break;
                    }
                }
            }

            for index in (num_1_index..line.len()).rev() {
                let line = &line[index..];
                if let Some(digit) = get_digit_text(line) {
                    nums.1 = Some(digit);
                    break;
                } else {
                    let letter = line.as_bytes()[0];
                    if letter.is_ascii_digit() {
                        nums.1 = Some(letter  - 48);
                        break;
                    }
                }
            }

            ((nums.0 * 10) + nums.1.unwrap_or(nums.0)) as u32
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_answer() {

        let data = r#"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"#;

        assert_eq!(calc_answer(data), 281);

    }
}
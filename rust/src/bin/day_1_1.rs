use std::fs;

fn main() {

    let data = fs::read_to_string("..\\inputs\\day_1.txt")
        .expect("couldn't read file to string");

    let answer = calc_answer(&data);
    println!("{answer}");

    assert_eq!(answer, 54697);
}

fn calc_answer(data: &str) -> u32 {

    let answer = data.lines()
        .map(|line| line.bytes())
        .map(|line| {
            let mut nums = (0_u8, None);

            let mut line = line.into_iter();

            for byte in line.by_ref() {
                if byte.is_ascii_digit() {
                    nums.0 = byte - 48;
                    break;
                }
            }

            for byte in line.rev() {
                if byte.is_ascii_digit() {
                    nums.1 = Some(byte - 48);
                    break;
                }
            }

            ((nums.0 * 10) + nums.1.unwrap_or(nums.0)) as u32
        })
        .sum::<u32>();

    answer
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_answer() {

        let data = r#"1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"#;

        assert_eq!(calc_answer(data), 142);

    }
}
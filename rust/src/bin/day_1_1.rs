use std::fs;

fn main() {

    let data = fs::read_to_string("..\\inputs\\day_1.txt")
        .expect("couldn't read file to string");

    let answer = data.lines()
        .map(|line| line.bytes())
        .map(|line| {
            let mut nums = (0_u8, None);

            let mut line = line.into_iter();

            while let Some(byte) = line.next() {
                if byte.is_ascii_digit() {
                    nums.0 = byte - 48;
                    break;
                }
            }

            while let Some(byte) = line.next() {
                if byte.is_ascii_digit() {
                    nums.1 = Some(byte - 48);
                }
            }

            format!("{}{}", nums.0, nums.1.unwrap_or(nums.0)).parse::<u32>().unwrap()
        })
        .fold(0_u32, |sum, next| {
            println!("{next}");
            sum + next
        });

    println!("{answer}");

}
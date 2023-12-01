use std::fs;

fn main() {
    println!("{}", calc_answer());
}

fn calc_answer() -> u32 {

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

            ((nums.0 * 10) + nums.1.unwrap_or(nums.0)) as u32
        })
        .fold(0_u32, |sum, next| {
            println!("{next}");
            sum + next
        });

    answer
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_answer() {
        assert_eq!(calc_answer(), 54697);
    }
}
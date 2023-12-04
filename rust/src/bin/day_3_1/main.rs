mod board;

use std::fs;
use crate::board::Board;

fn main() {
    let data = fs::read_to_string("..\\inputs\\day_3.txt")
        .expect("couldn't read file to string");

    let answer = calc_answer(data);

    print!("{answer}");
}

fn calc_answer(data: String) -> u64 {

    let mut board = Board::new();

    for line in data.lines() {
        board.add_row(line);
    }

    board.calc_valid()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_answer() {

        let data = r#"467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#.to_string();

        assert_eq!(calc_answer(data), 4361);
    }
}
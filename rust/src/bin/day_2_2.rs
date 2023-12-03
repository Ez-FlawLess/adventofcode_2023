use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("..\\inputs\\day_2.txt")
        .expect("couldn't read file to string");

    let answer = calc_answer(data);

    print!("{answer}");

    assert_eq!(answer, 72596);
}

fn calc_answer(data: String) -> usize {

    data.lines()
        .map(|line| {

            let line = line[(line.find(':').unwrap() + 2)..]
                .split("; ")
                .collect::<Vec<_>>();

            let mut game: HashMap<&str, u8> = HashMap::with_capacity(line.len());

            for set in line {
                for cube in set.split(", ") {
                    let cube = cube.splitn(2, ' ').collect::<Vec<_>>();

                    let new_count = cube[0].parse::<u8>().unwrap();

                    game.entry(cube[1])
                        .and_modify(|count| {
                            if new_count > *count {
                                *count = new_count;
                            }
                        })
                        .or_insert(new_count);
                }
            }

            game.into_values()
                .map(|count| count as usize)
                .product::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_answer() {

        let data = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#.to_string();

        assert_eq!(calc_answer(data), 2286);
    }
}
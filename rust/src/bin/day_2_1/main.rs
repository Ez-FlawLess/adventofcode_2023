use std::fs;

fn main() {
    let data = fs::read_to_string("..\\inputs\\day_2.txt")
        .expect("couldn't read file to string");

    let bag = vec![
        Cube::new("red".to_string(), 12),
        Cube::new("green".to_string(), 13),
        Cube::new("blue".to_string(), 14),
    ];

    let answer = calc_answer(data, bag);

    print!("{answer}");

    assert_eq!(answer, 2061);
}

struct Cube {
    name: String,
    count: u8,
}

impl Cube {
    fn new(name: String, count: u8) -> Self {
        Cube { name, count }
    }
}

fn calc_answer(data: String, bag: Vec<Cube>) -> usize {

    data.lines()
        .enumerate()
        .map(|(index, line)| {
            (index + 1, line)
        })
        .filter_map(|(id, line)| {

            for set in line[(line.find(':').unwrap() + 2)..].split("; ") {
                for cube in set.split(", ") {
                    let cube = cube.splitn(2, ' ').collect::<Vec<_>>();

                    let count = bag.iter().find(|c| {
                        c.name == cube[1]
                    }).unwrap().count;

                    if count < cube[0].parse::<u8>().unwrap() {
                        return None;
                    }
                }
            }

            Some(id)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_answer() {

        // 12 red cubes, 13 green cubes, and 14 blue cubes
        let bag = vec![
            Cube::new("red".to_string(), 12),
            Cube::new("green".to_string(), 13),
            Cube::new("blue".to_string(), 14),
        ];

        let data = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#.to_string();

        assert_eq!(calc_answer(data, bag), 8);
    }
}
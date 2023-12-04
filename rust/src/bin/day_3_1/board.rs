#[derive(Debug, PartialEq, Eq)]
enum Place {
    Number,
    Symbol,
    Empty,
}

#[derive(Debug)]
pub struct Board {
    board: Vec<Vec<Place>>,
    nums: Vec<Num>,
}

impl Board {
    pub fn new() -> Self {
        Self { board: Vec::new(), nums: Vec::new() }
    }

    pub fn add_row(&mut self, line: &str) {

        let mut bytes = line.bytes().enumerate();

        let mut row = Vec::with_capacity(line.len());

        'outer: while let Some((index, byte)) = bytes.next() {
            match byte {
                b'0'..=b'9' => {
                    row.push(Place::Number);
                    let from = index;
                    for (index, byte) in bytes.by_ref() {
                        if let b'0'..=b'9' = byte {
                            row.push(Place::Number);
                        } else {
                            let to = index - 1;
                            self.nums.push(Num::new(&line[from..=to], self.board.len(), from, to));
                            if b'.' == byte {
                                row.push(Place::Empty);
                            } else {
                                row.push(Place::Symbol);
                            }
                            continue 'outer;
                        }
                    }
                    let to = line.len() - 1;
                    self.nums.push(Num::new(&line[from..=to], self.board.len(), from, to));
                },
                b'.' => {
                    row.push(Place::Empty);
                },
                _ => {
                    row.push(Place::Symbol);
                },
            };
        }

        self.board.push(row);
    }

    pub fn calc_valid(self) -> u64 {
        let mut total = 0_u64;

        let mut add = |x: u64| {
            total = total.checked_add(x).unwrap();
        };

        'nums: for num in self.nums {

            let from = if let Some(from) = num.from.checked_sub(1) {

                if let Some(place) = self.board[num.row].get(from) {
                    if Place::Symbol == *place {
                        //total += num.value as u64;
                        add(num.value as u64);
                        continue 'nums;
                    }
                }

                from
            } else {
                num.from
            };
            let to = num.to + 1;
            if let Some(place) = self.board[num.row].get(to) {
                if Place::Symbol == *place {
                    //total += num.value as u64;
                    add(num.value as u64);
                    continue 'nums;
                }
            }
            // upper row
            if num.row != 0 {
                let row = &self.board[num.row - 1];
                for index in from..=to {
                    if let Some(place) = row.get(index) {
                        if Place::Symbol == *place {
                            //total += num.value as u64;
                            add(num.value as u64);
                            continue 'nums;
                        }
                    }
                }
            }

            if num.row != self.board.len() - 1 {
                let row = &self.board[num.row + 1];
                for index in from..=to {
                    if let Some(place) = row.get(index) {
                        if Place::Symbol == *place {
                            //total += num.value as u64;
                            add(num.value as u64);
                            continue 'nums;
                        }
                    }
                }
            }
        }

        total
    }
}

#[derive(Debug)]
struct Num {
    value: u16,
    row: usize,
    from: usize,
    to: usize,
}

impl Num {
    fn new(value: &str, row: usize, from: usize, to: usize) -> Self {
        Self {
            value: value.parse().unwrap(),
            row,
            from,
            to,
        }
    }
}
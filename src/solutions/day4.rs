use crate::AoCDay;

pub struct Code;

#[derive(Debug, Clone, Copy)]
enum Spot {
    Hit(i32),
    NoHit(i32),
}

impl Spot {
    fn num(&self) -> i32 {
        match self {
            Spot::Hit(n) => *n,
            Spot::NoHit(n) => *n,
        }
    }
    fn is_hit(&self) -> bool {
        matches!(self, Spot::Hit(_))
    }
}

impl Default for Spot {
    fn default() -> Self {
        Spot::NoHit(0)
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Board {
    rows: [[Spot; 5]; 5],
    columns: [[Spot; 5]; 5],
}

impl Board {
    fn from_line(&mut self, line: &str, row: usize) {
        for (column, num) in line
        .trim()
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| num.parse::<i32>().unwrap())
        .enumerate()
    {
        self.rows[row][column] = Spot::NoHit(num);
        self.columns[column][row] = Spot::NoHit(num);
    }
    }
    fn sum_of_unmarked(&self) -> i32 {
        self.rows
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|s| !s.is_hit())
                    .map(|s| s.num())
                    .sum::<i32>()
            })
            .sum()
    }
    fn mark_hit(&mut self, guess: i32) {
        for row in 0..5 {
            for column in 0..5 {
                match self.rows[row][column] {
                    Spot::NoHit(cell_num) if cell_num == guess => {
                        self.rows[row][column] = Spot::Hit(cell_num)
                    }
                    _ => {}
                }
                match self.columns[column][row] {
                    Spot::NoHit(cell_num) if cell_num == guess => {
                        self.columns[column][row] = Spot::Hit(cell_num)
                    }
                    _ => {}
                }
            }
        }
    }
    fn check(&self) -> bool {
        self.rows.iter().any(|row| row.iter().all(|s| s.is_hit()))
            || self.columns.iter().any(|row| row.iter().all(|s| s.is_hit()))
    }
}

fn check_boards(boards: &[Board]) -> Option<Board> {
    for board in boards {
        if board.check() {
            return Some(*board);
        }
    }
    None
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_argss: &[String]) -> String {
        let input = input.lines().collect::<Vec<_>>();
        let numbers = input[0];

        let mut boards: Vec<Board> = vec![];
        let mut board: Board = Default::default();
        let mut row = 0;
        for line in input.iter().skip(1).skip_while(|line| line.is_empty()) {
            if line.is_empty() {
                boards.push(board);
                board = Default::default();
                row = 0;
            } else {
                board.from_line(line, row);
                row += 1;
            }
        }

        for quess in numbers.split(',').collect::<Vec<_>>() {
            let guess = quess.parse::<i32>().unwrap();
            for boards_index in 0..boards.len() {
                boards[boards_index].mark_hit(guess);
            }
            if let Some(board) = check_boards(&boards) {
                let sum_of_unmarked = board.sum_of_unmarked();
                let answer = sum_of_unmarked * guess;
                assert_eq!(answer, 38594);
                return format!("{}", answer); // 38594/~270μs
            }
        }

        panic!("At the disco")
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let input = input.lines().collect::<Vec<_>>();
        let numbers = input[0];

        let mut boards: Vec<Board> = vec![];
        let mut board: Board = Default::default();
        let mut row = 0;
        for line in input.iter().skip(1).skip_while(|line| line.is_empty()) {
            if line.is_empty() {
                boards.push(board);
                board = Default::default();
                row = 0;
            } else {
                board.from_line(line, row);
                row += 1;
            }
        }

        for quess in numbers.split(',').collect::<Vec<_>>() {
            let guess = quess.parse::<i32>().unwrap();
            for boards_index in 0..boards.len() {
                boards[boards_index].mark_hit(guess);
            }
            let mut boards_len = boards.len();
            if boards_len > 1 {
                boards = boards
                    .into_iter()
                    .filter(|board| {
                        if board.check() && boards_len > 1 {
                            boards_len -= 1;
                            false
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<Board>>();
            } else if boards[0].check() {
                let sum_of_unmarked: i32 = boards[0].sum_of_unmarked();

                let answer = sum_of_unmarked * guess;
                assert_eq!(answer, 21184);
                return format!("{}", sum_of_unmarked * guess); // 21184/~970μs
            }
        }

        panic!("At the disco")
    }

    fn both(&self, input: &str, extra_args: &[String]) -> String {
        let p1 = self.part1(input, extra_args);
        let p2 = self.part2(input, extra_args);
        format!(
            "Part1: {}\n\
            Part2: {}",
            p1, p2
        )
    }
}

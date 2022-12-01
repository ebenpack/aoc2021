use crate::AoCDay;

use nalgebra::{matrix, SMatrix};

pub struct Code;

struct Board {
    board: SMatrix<i8, 5, 5>,
    row_counts: [u8; 5],
    column_counts: [u8; 5],
}

impl Board {
    #[inline(always)]
    fn check(&self) -> bool {
        matches!(
            self.row_counts.iter().find(|&row_count| *row_count == 5),
            Some(_)
        ) || matches!(
            self.column_counts
                .iter()
                .find(|&column_count| *column_count == 5),
            Some(_)
        )
    }
    #[inline(always)]
    fn sum_of_unmarked(&self) -> i32 {
        self.board
            .iter()
            .filter(|&n| *n != -1)
            .map(|n| i32::from(*n))
            .sum()
    }
}

#[inline(always)]
fn check_boards(boards: &[Board]) -> Option<i32> {
    for board in boards {
        if board.check() {
            return Some(board.sum_of_unmarked());
        }
    }
    None
}

#[inline(always)]
fn mark_hits(boards: &mut [Board], number: i8) {
    for board in boards.iter_mut() {
        for (i, cell) in board.board.iter_mut().enumerate() {
            if *cell == number {
                *cell = -1;
                let x = i % 5;
                let y = (i / 5) % 5;
                board.row_counts[x] += 1;
                board.column_counts[y] += 1;
            }
        }
    }
}

#[inline(always)]
fn numbers_and_boards_from_input(input: &str) -> (&str, Vec<Board>) {
    let input = input.lines().collect::<Vec<_>>();
    let numbers = input[0];
    let mut boards = vec![];
    let mut board = matrix![
        0,0,0,0,0;
        0,0,0,0,0;
        0,0,0,0,0;
        0,0,0,0,0;
        0,0,0,0,0
    ];
    let mut row = 0;
    for line in input.iter().skip(1).skip_while(|line| line.is_empty()) {
        if line.is_empty() {
            boards.push(Board {
                board,
                row_counts: [0; 5],
                column_counts: [0; 5],
            });
            board = matrix![
                0,0,0,0,0;
                0,0,0,0,0;
                0,0,0,0,0;
                0,0,0,0,0;
                0,0,0,0,0
            ];
            row = 0;
        } else {
            for (column, num) in line
                .trim()
                .split(' ')
                .filter(|num| !num.is_empty())
                .map(|num| num.parse::<i8>().unwrap())
                .enumerate()
            {
                board[(row, column)] = num;
            }
            row += 1;
        }
    }
    (numbers, boards)
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let (numbers, mut boards) = numbers_and_boards_from_input(input);

        for number in numbers.split(',') {
            let number = number.parse::<i8>().unwrap();
            mark_hits(&mut boards, number);
            if let Some(sum_of_unmarked) = check_boards(&boards) {
                let answer = sum_of_unmarked * i32::from(number);
                debug_assert_eq!(answer, 38594);
                return format!("{}", answer); // 38594/~250μs
            }
        }

        panic!("At the disco")
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let (numbers, mut boards) = numbers_and_boards_from_input(input);

        for number in numbers.split(',') {
            let number = number.parse::<i8>().unwrap();
            mark_hits(&mut boards, number);

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
                let sum_of_unmarked = boards[0].sum_of_unmarked();

                let answer = sum_of_unmarked * i32::from(number);
                debug_assert_eq!(answer, 21184);
                return format!("{}", answer); // 21184/~440μs
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

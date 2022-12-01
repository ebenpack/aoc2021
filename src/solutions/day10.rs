#![allow(unused_variables)]
use crate::AoCDay;

pub struct Code;

fn line_is_corrupted(line: &str) -> i32 {
    let mut stack = vec![];
    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else {
            let matching = stack.pop().unwrap();
            match (matching, c) {
                ('(', ')') => {}
                ('[', ']') => {}
                ('{', '}') => {}
                ('<', '>') => {}
                (_, ')') => return 3,
                (_, ']') => return 57,
                (_, '}') => return 1197,
                (_, '>') => return 25137,
                _ => panic!("At the disco"),
            }
        }
    }
    0
}

fn line_is_incomplete(line: &str) -> i64 {
    let mut stack = vec![];
    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else {
            let matching = stack.pop().unwrap();
            match (matching, c) {
                ('(', ')') => {}
                ('[', ']') => {}
                ('{', '}') => {}
                ('<', '>') => {}
                (_, ')') => return 0,
                (_, ']') => return 0,
                (_, '}') => return 0,
                (_, '>') => return 0,
                _ => panic!("At the disco"),
            }
        }
    }
    let mut tot: i64 = 0;
    for next in stack.iter().rev() {
        tot *= 5;
        tot += match next {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => {
                panic!("At the disco")
            }
        }
    }
    tot
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let answer: i32 = input.lines().map(line_is_corrupted).sum();
        debug_assert_eq!(answer, 442131);
        format!("{}", answer) // 442131/~135μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let mut scores = input
            .lines()
            .map(line_is_incomplete)
            .filter(|x| *x > 0)
            .collect::<Vec<_>>();
        scores.sort_unstable();

        let answer = scores[scores.len() / 2];

        debug_assert_eq!(answer, 3646451424);
        format!("{}", answer) // 3646451424/~145μs
    }
}

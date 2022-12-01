use std::{
    fmt::{self, Display},
    iter::Sum,
    ops::Add,
};

use itertools::Itertools;
use rayon::prelude::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    sequence::{delimited, separated_pair},
    IResult,
};

use crate::AoCDay;

pub struct Code;

fn parse_number(input: &str) -> IResult<&str, SnailFish> {
    let (input, num) = digit1(input)?;
    let num = num.trim().parse::<i32>().unwrap();
    Ok((input, SnailFish::Number(num)))
}

fn parse_pair(input: &str) -> IResult<&str, SnailFish> {
    let (input, (a, b)) = delimited(
        tag("["),
        separated_pair(
            alt((parse_number, parse)),
            tag(","),
            alt((parse_number, parse)),
        ),
        tag("]"),
    )(input)?;
    Ok((input, SnailFish::Pair(Box::new(a), Box::new(b))))
}

fn parse(input: &str) -> IResult<&str, SnailFish> {
    let (input, pairs) = alt((parse_number, parse_pair))(input)?;
    Ok((input, pairs))
}

#[derive(Debug, Clone)]
enum SnailFish {
    Number(i32),
    Pair(Box<SnailFish>, Box<SnailFish>),
}

impl Add for SnailFish {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        SnailFish::Pair(Box::new(self), Box::new(rhs)).reduce()
    }
}

impl Add for &SnailFish {
    type Output = SnailFish;
    fn add(self, rhs: Self) -> Self::Output {
        SnailFish::Pair(Box::new(self.clone()), Box::new(rhs.clone())).reduce()
    }
}

impl Sum<SnailFish> for SnailFish {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        // YOLO
        iter.reduce(|a, b| a + b).unwrap()
    }
}

#[derive(Debug)]
enum Side {
    Unknown,
    PrevLeft,
    NextRight,
    Neither,
}

impl SnailFish {
    fn from_line(line: &str) -> Self {
        parse(line).unwrap().1
    }
    fn magnitude(&self) -> i32 {
        match self {
            SnailFish::Number(n) => *n,
            SnailFish::Pair(left, right) => (3 * left.magnitude()) + (2 * right.magnitude()),
        }
    }
    fn add_left(&mut self, num: i32) {
        match self {
            SnailFish::Number(n) => *self = SnailFish::Number(*n + num),
            SnailFish::Pair(left, _) => left.add_left(num),
        }
    }
    fn add_right(&mut self, num: i32) {
        match self {
            SnailFish::Number(n) => *self = SnailFish::Number(*n + num),
            SnailFish::Pair(_, right) => right.add_right(num),
        }
    }
    fn add_next_right(&mut self, num: i32, thing: bool) {
        match self {
            SnailFish::Number(n) => *self = SnailFish::Number(*n + num),
            SnailFish::Pair(left, right) => {
                if thing {
                    right.add_next_right(num, false);
                } else {
                    left.add_left(num);
                }
            }
        }
    }
    fn add_prev_left(&mut self, num: i32, thing: bool) {
        match self {
            SnailFish::Number(n) => *self = SnailFish::Number(*n + num),
            SnailFish::Pair(left, right) => {
                if thing {
                    left.add_prev_left(num, false);
                } else {
                    right.add_right(num);
                }
            }
        }
    }
    fn reduce(mut self) -> Self {
        loop {
            if self.explode(1).is_some() || self.split() {
                continue;
            }
            break;
        }
        self
    }
    fn split(&mut self) -> bool {
        match self {
            SnailFish::Number(n) => {
                if *n >= 10 {
                    *self = SnailFish::Pair(
                        Box::new(SnailFish::Number((*n as f64 / 2.0).floor() as i32)),
                        Box::new(SnailFish::Number((*n as f64 / 2.0).ceil() as i32)),
                    );
                    true
                } else {
                    false
                }
            }
            SnailFish::Pair(left, right) => left.split() || right.split(),
        }
    }
    fn explode(&mut self, depth: i32) -> Option<(Side, i32, i32)> {
        if depth > 4 {
            let matched = match &self {
                SnailFish::Pair(left, right) => {
                    if let (SnailFish::Number(left_num), SnailFish::Number(right_num)) =
                        (left.as_ref(), right.as_ref())
                    {
                        Some((*left_num, *right_num))
                    } else {
                        None
                    }
                }
                SnailFish::Number(_) => None,
            };
            // This is here to avoid some move issues
            if let Some((left, right)) = matched {
                *self = SnailFish::Number(0);
                return Some((Side::Unknown, left, right));
            }
        }
        if let SnailFish::Pair(left, _) = self {
            if let Some((mutated, left_num, right_num)) = left.explode(depth + 1) {
                match mutated {
                    Side::Unknown => {
                        self.add_next_right(right_num, true);
                        return Some((Side::PrevLeft, left_num, right_num));
                    }
                    Side::NextRight => {
                        self.add_next_right(right_num, true);
                        return Some((Side::Neither, left_num, right_num));
                    }
                    _ => {
                        return Some((mutated, left_num, right_num));
                    }
                }
            }
        };
        if let SnailFish::Pair(_, right) = self {
            if let Some((mutated, left_num, right_num)) = right.explode(depth + 1) {
                match mutated {
                    Side::Unknown => {
                        self.add_prev_left(left_num, true);
                        return Some((Side::NextRight, left_num, right_num));
                    }
                    Side::PrevLeft => {
                        self.add_prev_left(left_num, true);
                        return Some((Side::Neither, left_num, right_num));
                    }
                    _ => {
                        return Some((mutated, left_num, right_num));
                    }
                }
            }
        };
        None
    }
}

impl Display for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match &self {
            SnailFish::Number(n) => format!("{}", n),
            SnailFish::Pair(left, right) => format!("[{},{}]", left, right),
        };
        write!(f, "{}", val)
    }
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let answer = input
            .lines()
            .map(SnailFish::from_line)
            .sum::<SnailFish>()
            .magnitude();

        debug_assert_eq!(answer, 3699);
        format!("{}", answer) // 3699/3500μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {    
        let inputs = input
            .lines()
            .map(SnailFish::from_line)
            .combinations(2)
            .collect::<Vec<_>>();

        let answer = inputs.par_iter().map(|combos| {
            let mag_one = (combos[0].clone() + combos[1].clone()).magnitude();
            let mag_two = (combos[1].clone() + combos[0].clone()).magnitude();
            mag_one.max(mag_two)
        }).max().unwrap();

        debug_assert_eq!(answer, 4735);
        format!("{}", answer) //4735/26896μs
    }
}

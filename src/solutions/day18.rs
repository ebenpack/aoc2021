use std::{
    fmt::{self, Display},
    iter::Sum,
    ops::Add,
};

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
    // fn print_tree(&self) -> String {
    //     let mut buffer = vec![];
    //     self.print_tree_helper("", "", &mut buffer);
    //     buffer.join("")
    // }
    // fn print_tree_helper(&self, prefix: &str, children_prefix: &str, buffer: &mut Vec<String>) {
    //     buffer.push(prefix.to_string());
    //     if let SnailFish::Number(n) = self {
    //         buffer.push(format!("{}", n));
    //     }
    //     buffer.push("\n".to_string());
    //     if let SnailFish::Pair(left, right) = self {
    //         left.print_tree_helper(
    //             &format!("{}├── ", children_prefix),
    //             &format!("{}│   ", children_prefix),
    //             buffer,
    //         );
    //         right.print_tree_helper(
    //             &format!("{}└── ", children_prefix),
    //             &format!("{}    ", children_prefix),
    //             buffer,
    //         );
    //     }
    // }
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_explode() {
        let tests = [
            ("[[[[[9,8],1],2],3],4],", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
            (
                "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
                "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            ),
            (
                "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
                // "[[[[4,0],[5,0]],[[0,[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
                "[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
                // [[[[4,0],[5,4]],[[0,[2,11]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]
            ),
        ];

        for (input, expected) in tests {
            let mut snail = SnailFish::from_line(input);
            snail.explode(1);
            assert_eq!(format!("{}", snail), expected);
        }
    }

    #[test]
    fn test_split() {
        let tests = [
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ];

        for (input, expected) in tests {
            let mut snail = SnailFish::from_line(input);
            snail.split();
            assert_eq!(format!("{}", snail), expected);
        }
    }
    #[test]
    fn test_magnitude() {
        let tests = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ];
        for (input, expected) in tests {
            assert_eq!(SnailFish::from_line(input).magnitude(), expected);
        }
    }
    #[test]
    fn test_add() {
        let tests = [
    (
        "[1,1]\n[2,2]\n[3,3]\n[4,4]",
        "[[[[1,1],[2,2]],[3,3]],[4,4]]"
    ),
    (
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
    ),
    (
        "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]\n[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
        "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
    ),
    (
        "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]\n[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
        "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
    ),
    (
        "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]\n[7,[5,[[3,8],[1,4]]]]",
        "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
    ),
    (
        "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]\n[[2,[2,2]],[8,[8,1]]]",
        "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"
    ),
    (
        "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]\n[2,9]",
        "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"
    ),
    (
        "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]\n[1,[[[9,3],9],[[9,0],[0,7]]]]",
        "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"
    ),
    (
        "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]\n[[[5,[7,4]],7],1]",
        "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"
    ),
    (
        "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]\n[[[[4,2],2],6],[8,7]]",
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
    )
        ];
        for (input, expected) in tests {
            let answer = input
                .lines()
                .map(|line| {
                    let (_, pairs) = parse(line).unwrap();
                    // pairs.reduce()
                    pairs
                })
                .sum::<SnailFish>();
            assert_eq!(format!("{}", answer), expected);
        }
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

        format!("{}", answer)
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let mut max = 0;
        let inputs = input
            .lines()
            .map(SnailFish::from_line)
            .collect::<Vec<_>>();
        
        for x in inputs.iter() {
            for y in inputs.iter() {
                let mag_one = (x + y).magnitude();
                let mag_two = (y + x).magnitude();
                max = max.max(mag_one).max(mag_two);
            }
        }

        let answer = max;
        format!("{}", answer)
    }
}

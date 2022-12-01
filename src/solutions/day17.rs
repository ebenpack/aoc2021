use std::ops::Neg;

use nom::{bytes::complete::tag, character::complete::digit1, combinator::opt, IResult};

use crate::AoCDay;

type Coord = (i32, i32);

fn parse_target(input: &str) -> IResult<&str, (Coord, Coord)> {
    let (input, _) = tag("target area: x=")(input)?;
    let (input, x1_neg) = opt(tag("-"))(input)?;
    let (input, x1) = digit1(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, x2_neg) = opt(tag("-"))(input)?;
    let (input, x2) = digit1(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y1_neg) = opt(tag("-"))(input)?;
    let (input, y1) = digit1(input)?;
    let (input, _) = tag("..")(input)?;
    let (input, y2_neg) = opt(tag("-"))(input)?;
    let (input, y2) = digit1(input)?;

    let x1 = x1.trim().parse::<i32>().unwrap();
    let x2 = x2.trim().parse::<i32>().unwrap();

    let x1 = if x1_neg.is_some() { x1.neg() } else { x1 };
    let x2 = if x2_neg.is_some() { x2.neg() } else { x2 };

    let y1 = y1.trim().parse::<i32>().unwrap();
    let y2 = y2.trim().parse::<i32>().unwrap();

    let y1 = if y1_neg.is_some() { y1.neg() } else { y1 };
    let y2 = if y2_neg.is_some() { y2.neg() } else { y2 };

    Ok((input, ((x1.min(x2), x2.max(x1)), (y1.min(y2), y2.max(y1)))))
}

pub struct Code;

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let (_, ((x1, x2), (y1, y2))) = parse_target(input).unwrap();

        let mut max_height = 0;
        for x_vel in 1..=100 {
            for y_vel in 1..=200 {
                let mut x_vel = x_vel;
                let mut y_vel = y_vel;
                let mut pos = (0, 0);
                let mut possible_max_height = 0;
                'here: while pos.0 <= x2 && pos.1 >= y1 {
                    pos.0 += x_vel;
                    pos.1 += y_vel;
                    x_vel = x_vel
                        - match x_vel {
                            n if n == 0 => 0,
                            n if n < 0 => -1,
                            n if n > 0 => 1,
                            _ => unreachable!(),
                        };
                    y_vel -= 1;
                    possible_max_height = possible_max_height.max(pos.1);
                    if pos.0 >= x1 && pos.0 <= x2 && pos.1 >= y1 && pos.1 <= y2 {
                        max_height = max_height.max(possible_max_height);
                        break 'here;
                    }
                }
            }
        }

        let answer = max_height;
        debug_assert_eq!(answer, 19503);
        format!("{}", answer) // 19503/~1500μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let (_, ((x1, x2), (y1, y2))) = parse_target(input).unwrap();

        let mut number_of_hits = 0;
        for x_vel in 1..=200 {
            for y_vel in -200..=200 {
                let mut x_vel = x_vel;
                let mut y_vel = y_vel;
                let mut pos = (0, 0);
                let mut possible_max_height = 0;
                'here: while pos.0 <= x2 && pos.1 >= y1 {
                    pos.0 += x_vel;
                    pos.1 += y_vel;
                    x_vel = x_vel
                        - match x_vel {
                            n if n == 0 => 0,
                            n if n < 0 => -1,
                            n if n > 0 => 1,
                            _ => unreachable!(),
                        };
                    y_vel -= 1;
                    possible_max_height = possible_max_height.max(pos.1);
                    if pos.0 >= x1 && pos.0 <= x2 && pos.1 >= y1 && pos.1 <= y2 {
                        number_of_hits += 1;
                        break 'here;
                    }
                }
            }
        }

        let answer = number_of_hits;
        debug_assert_eq!(answer, 5200);
        format!("{}", answer) // 5200/~1500μs
    }
}

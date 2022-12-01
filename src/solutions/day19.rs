use std::ops::Neg;

use itertools::Itertools;
use std::collections::BTreeMap;
use nom::{bytes::complete::tag, character::complete::digit1, combinator::opt, IResult};

use crate::AoCDay;

pub struct Code;

type Point = (i32, i32, i32);

#[derive(Debug)]
struct Scanner {
    header: i32,
    points: Vec<Point>,
    distances: BTreeMap<f64, (Point,Point)>,
}

fn parse_header(input: &str) -> IResult<&str, i32> {
    let (input, _) = tag("--- scanner ")(input)?;
    let (input, header) = digit1(input)?;
    Ok((input, header.parse::<i32>().unwrap()))
}

fn convert_num(num: &str, neg: Option<&str>) -> i32 {
    let num = num.parse::<i32>().unwrap();
    if neg.is_some() {
        num.neg()
    } else {
        num
    }
}

fn parse_coords(input: &str) -> IResult<&str, (i32, i32, i32)> {
    let (input, x_neg) = opt(tag("-"))(input)?;
    let (input, x) = digit1(input)?;
    let (input, _) = tag(",")(input)?;

    let (input, y_neg) = opt(tag("-"))(input)?;
    let (input, y) = digit1(input)?;
    let (input, _) = tag(",")(input)?;

    let (input, z_neg) = opt(tag("-"))(input)?;
    let (input, z) = digit1(input)?;

    Ok((
        input,
        (
            convert_num(x, x_neg),
            convert_num(y, y_neg),
            convert_num(z, z_neg),
        ),
    ))
}

fn sq(n: i32) -> f64 {
    (n * n) as f64
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let mut new_scanner = true;
        let mut scanners = vec![];
        let mut current_scanner = Scanner {
            header: 0,
            points: vec![],
            distances: BTreeMap::new(),
        };
        for line in input.lines() {
            if line.is_empty() {
                new_scanner = true;
                let distances = current_scanner
                    .points
                    .iter()
                    .combinations(2)
                    .map(|combos| {
                        let p1@(x1,y1,z1) = combos[0];
                        let p2@(x2,y2,z2) = combos[1];
                        (f64::sqrt(
                            sq(x2 -x1) + sq(y2 -y1) + sq(z2 -z1)
                        ), (p1, p2))
                        
                    })
                    .collect::<BTreeMap<_,_>>();
                current_scanner.distances = distances;
                scanners.push(current_scanner);
                current_scanner = Scanner {
                    header: 0,
                    points: vec![],
                    distances: BTreeMap::new(),
                };
                continue;
            }
            if new_scanner {
                let (_, header) = parse_header(line).unwrap();
                current_scanner.header = header;
                new_scanner = false;
            } else {
                let (_, (x, y, z)) = parse_coords(line).unwrap();
                current_scanner.points.push((x, y, z));
            }
        }
        println!("{:?}", scanners);
        todo!()
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        todo!()
    }
}

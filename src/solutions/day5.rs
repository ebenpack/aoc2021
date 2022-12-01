use crate::AoCDay;

use hashbrown::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace1},
    IResult,
};

pub struct Code;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Coord,
    end: Coord,
}

struct CoordMap {
    coords: HashMap<i32, i32>,
    intersections: i32,
}

impl CoordMap {
    #[inline(always)]
    fn new() -> Self {
        CoordMap {
            coords: HashMap::with_capacity(500),
            intersections: 0,
        }
    }
    #[inline(always)]
    fn add(&mut self, x: i32, y: i32) {
        // Store the x coord in the bottom 16 bytes,
        // and the y coord in the top 16 bytes.
        // This seems to be sliiightly more performant
        // than using a tuple key.
        let key = x | y << 16;
        let count = self.coords.entry(key).or_insert(0);
        *count += 1;
        if *count == 2 {
            self.intersections += 1;
        }
    }
    #[inline(always)]
    fn count_of_intersections(&self) -> i32 {
        self.intersections
    }
}

#[inline(always)]
fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, x1) = digit1(input)?;
    let (input, _) = char(',')(input)?;
    let (input, y1) = digit1(input)?;

    let (input, _) = multispace1(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, x2) = digit1(input)?;
    let (input, _) = char(',')(input)?;
    let (input, y2) = digit1(input)?;

    Ok((
        input,
        Line {
            start: Coord {
                x: x1.parse::<i32>().unwrap(),
                y: y1.parse::<i32>().unwrap(),
            },
            end: Coord {
                x: x2.parse::<i32>().unwrap(),
                y: y2.parse::<i32>().unwrap(),
            },
        },
    ))
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let mut map = CoordMap::new();
        for line in input.lines() {
            let (_, line) = parse_line(line).unwrap();
            if line.start.x == line.end.x {
                let x = line.start.x;
                let start = line.start.y.min(line.end.y);
                let end = line.start.y.max(line.end.y);
                for y in start..=end {
                    map.add(x, y);
                }
            } else if line.start.y == line.end.y {
                let y = line.start.y;
                let start = line.start.x.min(line.end.x);
                let end = line.start.x.max(line.end.x);
                for x in start..=end {
                    map.add(x, y);
                }
            }
        }
        let answer = map.count_of_intersections();
        debug_assert_eq!(answer, 6007);
        format!("{}", answer) // 6007/~4200μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let mut map = CoordMap::new();
        for line in input.lines() {
            let (_, line) = parse_line(line).unwrap();
            if line.start.x == line.end.x {
                let x = line.start.x;
                let start = line.start.y.min(line.end.y);
                let end = line.start.y.max(line.end.y);
                for y in start..=end {
                    map.add(x, y);
                }
            } else if line.start.y == line.end.y {
                let y = line.start.y;
                let start = line.start.x.min(line.end.x);
                let end = line.start.x.max(line.end.x);
                for x in start..=end {
                    map.add(x, y);
                }
            } else {
                let (start, end) = if line.start.x < line.end.x {
                    (line.start, line.end)
                } else {
                    (line.end, line.start)
                };
                let slope = (end.y - start.y) / (end.x - start.x);
                let mut y = start.y;
                let y_step = if slope < 0 { -1 } else { 1 };
                for x in start.x..=end.x {
                    map.add(x, y);
                    y += y_step;
                }
            }
        }
        let answer = map.count_of_intersections();
        debug_assert_eq!(answer, 19349);
        format!("{}", answer) // 19349/~8500μs
    }
}

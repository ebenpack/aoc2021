use lazy_static::lazy_static;
use regex::Regex;

use crate::AoCDay;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    IResult,
};

pub struct Code;

struct Position {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32,
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, dir) = alt((tag("forward"), tag("down"), tag("up")))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, magnitude) = digit1(input)?;
    let num = magnitude.trim().parse::<i32>().unwrap();
    match dir {
        "forward" => Ok((input, Command::Forward(num))),
        "down" => Ok((input, Command::Down(num))),
        "up" => Ok((input, Command::Up(num))),
        _ => unreachable!(),
    }
}

#[allow(unused)]
fn parse_command_re(input: &str) -> Result<Command, ()> {
    // nom seems just as fast, but gonna leave this here as a quick
    // reference in case I need to do some regexing later
    lazy_static! {
        static ref RE: Regex = Regex::new("(forward|down|up)\\s\\d+").unwrap();
    }
    let capture = RE.captures(input).unwrap();
    let dir = capture.get(1).unwrap().as_str();
    let num = capture.get(2).unwrap().as_str();
    let num = num.trim().parse::<i32>().unwrap();
    match dir {
        "forward" => Ok(Command::Forward(num)),
        "down" => Ok(Command::Down(num)),
        "up" => Ok(Command::Up(num)),
        _ => unreachable!(),
    }
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_argss: &[String]) -> String {
        let mut pos = Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };

        for line in input.lines() {
            let (_, command) = parse_command(line).unwrap();

            match command {
                Command::Forward(n) => pos.horizontal += n,
                Command::Down(n) => pos.depth += n,
                Command::Up(n) => pos.depth -= n,
            }
        }
        let answer = pos.horizontal * pos.depth;
        debug_assert_eq!(answer, 2187380);
        format!("{}", pos.horizontal * pos.depth) // 2187380/~70μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let mut pos = Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        };

        for line in input.lines() {
            let (_, command) = parse_command(line).unwrap();
            match command {
                Command::Forward(n) => {
                    pos.horizontal += n;
                    pos.depth += pos.aim * n
                }
                Command::Down(n) => pos.aim += n,
                Command::Up(n) => pos.aim -= n,
            }
        }
        let answer = pos.horizontal * pos.depth;
        debug_assert_eq!(answer, 2086357770);
        format!("{}", answer) // 2086357770/~70μs
    }
}

use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::num::NonZeroU8;
use std::ops::Sub;
use std::path::{Path, PathBuf};
use std::str::FromStr;
extern crate time;
use time::Duration;
use time::Instant;

use color_eyre::eyre::Result;
use structopt::StructOpt;

use crate::day::Day;

#[derive(StructOpt)]
pub struct Run {
    /// Problem day to run
    day: Day,
    /// Path to input file
    #[structopt(long)]
    input: Option<PathBuf>,
    /// Any extra arguments in question
    #[structopt(long)]
    extra: Vec<String>,
    /// Part to run
    #[structopt(long, short, default_value)]
    part: Part,
    /// Benchmark
    #[structopt(long, short)]
    bench: bool,
}

#[derive(StructOpt)]
pub struct RunAll {
    /// End day
    #[structopt(long, short)]
    end: Option<NonZeroU8>,
    /// Benchmark
    #[structopt(long, short)]
    bench: bool,
}

#[derive(StructOpt)]
enum Part {
    Part1,
    Part2,
    Both,
}

impl FromStr for Part {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "part1" | "1" => Ok(Self::Part1),
            "part2" | "2" => Ok(Self::Part2),
            "both" | "b" => Ok(Self::Both),
            _ => Err("Unknown"),
        }
    }
}

impl Default for Part {
    fn default() -> Self {
        Self::Both
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Part1 => write!(f, "part1"),
            Part::Part2 => write!(f, "part2"),
            Part::Both => write!(f, "both"),
        }
    }
}

impl Run {
    pub fn run(&self) -> Result<(String, Option<Duration>)> {
        let mut file;
        if let Some(ref path) = self.input {
            file = File::open(path)?;
        } else {
            let path = env::var("AOC_INPUT")?;
            let path = Path::new(&path).join(format!("Day{}", self.day.get()));
            file = File::open(path)?;
        }

        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        let code = self.day.get_code();
        let extra_args = &self.extra;

        let start = Instant::now();
        let output = match self.part {
            Part::Part1 => code.part1(&input, extra_args),
            Part::Part2 => code.part2(&input, extra_args),
            Part::Both => code.both(&input, extra_args),
        };

        let mut duration = None;
        if self.bench {
            let end = Instant::now();
            duration = Some(end.sub(start));
        }

        Ok((output, duration))
    }
}

impl RunAll {
    pub fn run_all(&self) -> Result<(String, Option<Duration>)> {
        let mut output: Vec<String> = vec![];
        let start = Instant::now();
        let end = self
            .end
            .unwrap_or_else(|| NonZeroU8::new(25).unwrap())
            .into();
        for day in 1..=end {
            output.push(format!("Day {}", day));
            let day = NonZeroU8::new(day).unwrap();
            let day = Day { day };
            let path = env::var("AOC_INPUT")?;
            let path = Path::new(&path).join(format!("Day{}", day.day));
            let mut file = File::open(path)?;

            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            let code = day.get_code();
            let extra_args = vec![];

            let start = Instant::now();
            let result = code.both(&input, &extra_args);
            let end = Instant::now();
            let duration = end.sub(start);
            if self.bench {
                output.push(format!("Time: {}μs", duration.whole_microseconds()));
            }

            output.push(result);
            output.push("".to_string());
        }

        let mut duration = None;
        if self.bench {
            let end = Instant::now();
            duration = Some(end.sub(start));
        }

        Ok((output.join("\n"), duration))
    }
}

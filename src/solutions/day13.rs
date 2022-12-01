use nom::{branch::alt, bytes::complete::tag, character::complete::digit1, IResult};

use hashbrown::HashSet;

use crate::AoCDay;

pub struct Code;

#[derive(Debug)]
enum Fold {
    X(i32),
    Y(i32),
}

#[inline(always)]
fn parse_fold(input: &str) -> IResult<&str, Fold> {
    let (input, _) = tag("fold along ")(input)?;
    let (input, axis) = alt((tag("x"), tag("y")))(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, num) = digit1(input)?;
    let num = num.trim().parse::<i32>().unwrap();
    match axis {
        "x" => Ok((input, Fold::X(num))),
        "y" => Ok((input, Fold::Y(num))),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Mapping {
    map: HashSet<(i32, i32)>,
    max_x: i32,
    max_y: i32,
}

impl Mapping {
    #[inline(always)]
    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::X(x_trans) => {
                let mut moves = vec![];
                for (x, y) in self.map.iter() {
                    if x > x_trans {
                        let distance = x - x_trans;
                        moves.push(((*x, *y), (x_trans - distance, *y)));
                    }
                }
                for ((x, y), (new_x, new_y)) in moves {
                    self.map.remove(&(x, y));
                    self.map.insert((new_x, new_y));
                }
                self.max_x = *x_trans;
            }
            Fold::Y(y_trans) => {
                let mut moves = vec![];
                for (x, y) in self.map.iter() {
                    if y > y_trans {
                        let distance = y - y_trans;
                        moves.push(((*x, *y), (*x, y_trans - distance)));
                    }
                }
                for ((x, y), (new_x, new_y)) in moves {
                    self.map.remove(&(x, y));
                    self.map.insert((new_x, new_y));
                }
                self.max_y = *y_trans;
            }
        }
    }
    #[inline(always)]
    fn print(&self) -> String {
        let mut s = vec!['\n'];
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                if self.map.contains(&(x, y)) {
                    s.push('#');
                } else {
                    s.push(' ');
                }
            }
            s.push('\n');
        }
        s.iter().collect::<String>()
    }
}

#[inline(always)]
fn get_map_and_folds_from_input(input: &str) -> (Mapping, Vec<Fold>) {
    let mut first_part = true;
    let mut map = HashSet::new();
    let mut folds = vec![];
    let mut max_x = 0;
    let mut max_y = 0;
    for line in input.lines() {
        if line.is_empty() {
            first_part = false;
            continue;
        }
        if first_part {
            let parts = line.split(',').collect::<Vec<_>>();
            let x = parts[0].parse::<i32>().unwrap();
            let y = parts[1].parse::<i32>().unwrap();
            map.insert((x, y));
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        } else {
            let (_, fold) = parse_fold(line).unwrap();
            folds.push(fold);
        }
    }
    let map = Mapping { map, max_x, max_y };
    (map, folds)
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let (mut map, folds) = get_map_and_folds_from_input(input);
        map.fold(&folds[0]);
        let answer = map.map.len();

        debug_assert_eq!(answer, 724);
        format!("{}", answer) // 724/~340μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let (mut map, folds) = get_map_and_folds_from_input(input);
        for f in folds {
            map.fold(&f);
        }

        map.print() // CPJBERUL/~460μs
    }
}

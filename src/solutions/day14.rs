#![allow(unused_variables)]

use hashbrown::HashMap;

use crate::AoCDay;

pub struct Code;

#[inline(always)]
fn solve(input: &str, iterations: i32) -> i128 {
    let mut template = HashMap::new();
    let mut pairs = HashMap::new();
    let mut counts = HashMap::new();
    for line in input.lines() {
        if template.is_empty() {
            for char in line.chars() {
                let entry = counts.entry(char).or_insert(0);
                *entry += 1;
            }
            for chars in line.chars().zip(line.chars().skip(1)) {
                let entry = template.entry(chars).or_insert(0);
                *entry += 1;
            }
        } else if line.is_empty() {
            continue;
        } else {
            let p = line.split(" -> ").collect::<Vec<_>>();
            pairs.insert(
                (p[0].chars().next().unwrap(), p[0].chars().nth(1).unwrap()),
                p[1].chars().next().unwrap(),
            );
        }
    }

    for i in 0..iterations {
        let mut next = template.clone();
        for (p @ (c1, c2), val) in template.iter() {
            if let Some(insert) = pairs.get(p) {
                let entry = next.entry(*p).or_insert(0);
                *entry -= val;

                let entry = next.entry((*c1, *insert)).or_insert(0);
                *entry += val;

                let entry = next.entry((*insert, *c2)).or_insert(0);
                *entry += val;

                let entry = counts.entry(*insert).or_insert(0);
                *entry += val;
            }
        }
        template = next;
    }

    let mut most = 0;
    let mut least = i128::MAX;
    for (c, n) in counts {
        if n > most {
            most = n;
        }
        if n < least {
            least = n;
        }
    }

    most - least
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let answer = solve(input, 10);
        debug_assert_eq!(answer, 2590);
        format!("{}", answer) // 2590
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let answer = solve(input, 40);
        debug_assert_eq!(answer, 2875665202438);
        format!("{}", answer) // 2875665202438/
    }
}

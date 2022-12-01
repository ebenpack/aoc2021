#![allow(unused_variables)]

use hashbrown::HashSet;

use crate::AoCDay;

pub struct Code;

#[derive(Debug)]
struct Mapping {
    map: Vec<i32>,
    x_size: usize,
    y_size: usize,
    flashes: i64,
}

impl Mapping {
    #[inline(always)]
    fn from_str(input: &str) -> Self {
        let mut map = Vec::with_capacity(10 * 10);
        let mut y_size = 0;
        let mut x_size = 0;
        for (y, line) in input.lines().enumerate() {
            y_size = y_size.max(y + 1);
            for (x, num) in line.chars().enumerate() {
                x_size = x_size.max(x + 1);
                map.push(num.to_string().parse::<i32>().unwrap());
            }
        }
        Mapping {
            map,
            x_size,
            y_size,
            flashes: 0,
        }
    }
    #[inline(always)]
    fn get(&self, x: usize, y: usize) -> Option<i32> {
        if x >= self.x_size || y >= self.y_size {
            None
        } else {
            Some(self.map[x + (self.x_size * y)])
        }
    }
    #[inline(always)]
    fn incr(&mut self, x: usize, y: usize) {
        if x < self.x_size && y < self.y_size {
            self.map[x + (self.x_size * y)] += 1;
        }
    }
    #[inline(always)]
    fn flash(&mut self, to_visit: &mut Vec<(usize, usize)>) -> bool {
        let mut flashed = HashSet::new();

        while !to_visit.is_empty() {
            let (x, y) = to_visit.pop().unwrap();
            if let Some(n) = self.get(x, y) {
                if n > 9 && !flashed.contains(&(x, y)) {
                    self.flashes += 1;
                    flashed.insert((x, y));
                    for y1 in -1..=1 {
                        for x1 in -1..=1 {
                            if x1 == 0 && y1 == 0
                                || x == 0 && x1 == -1
                                || y == 0 && y1 == -1
                                || x == self.x_size - 1 && x1 == 1
                                || y == self.y_size - 1 && y1 == 1
                            {
                            } else {
                                let new_x = ((x as i32) + x1) as usize;
                                let new_y = ((y as i32) + y1) as usize;
                                self.incr(new_x, new_y);
                                to_visit.push((new_x, new_y));
                            }
                        }
                    }
                }
            }
        }

        flashed.len() == self.x_size * self.y_size
    }

    #[inline(always)]
    fn step(&mut self) -> bool {
        let mut to_flash = vec![];
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                self.incr(x, y);
                if self.get(x, y).unwrap() > 9 {
                    to_flash.push((x, y));
                }
            }
        }

        let all_flashed = if to_flash.is_empty() {
            false
        } else {
            let all_flashed = self.flash(&mut to_flash);
            for x in self.map.iter_mut() {
                if *x > 9 {
                    *x = 0;
                }
            }
            all_flashed
        };

        all_flashed
    }
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let mut map = Mapping::from_str(input);

        for _ in 0..100 {
            map.step();
        }
        let answer = map.flashes;
        debug_assert_eq!(answer, 1725);
        format!("{}", answer) // 1725/410μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let mut map = Mapping::from_str(input);

        let mut answer = 1;
        while !map.step() {
            answer += 1;
        }

        debug_assert_eq!(answer, 308);
        format!("{}", answer) // 1725/1100μs
    }
}

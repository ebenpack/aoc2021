use crate::AoCDay;

use hashbrown::HashMap;
use hashbrown::HashSet;

pub struct Code;

struct Mapping {
    map: HashMap<(usize, usize), i32>,
    x_size: usize,
    y_size: usize,
}

impl Mapping {
    // 
    // x = i % width;    // % is the "modulo operator", the remainder of i / width;
    // y = i / width; 
    fn from_str(input: &str) -> Self {
        let mut map = HashMap::new();
        let mut y_size = 0;
        let mut x_size = 0;
        for (y, line) in input.lines().enumerate() {
            y_size = y_size.max(y);
            for (x, num) in line.chars().enumerate() {
                x_size = x_size.max(x);
                map.insert((x, y), num.to_string().parse::<i32>().unwrap());
            }
        }
        Mapping {
            map,
            x_size,
            y_size,
        }
    }
    fn get(&self, x: usize, y: usize) -> Option<&i32> {
        self.map.get(&(x, y))
    }
    fn get_low_points(&self) -> Vec<((usize, usize), &i32)> {
        let mut low_points = vec![];
        for y in 0..=self.y_size {
            for x in 0..=self.x_size {
                let curr = self.get(x, y).unwrap();
                let up = if y == 0 {
                    &999
                } else {
                    self.get(x, y - 1).unwrap_or(&999)
                };
                let left = if x == 0 {
                    &999
                } else {
                    self.get(x - 1, y).unwrap_or(&999)
                };
                let down = self.get(x, y + 1).unwrap_or(&999);
                let right = self.get(x + 1, y).unwrap_or(&999);
                if curr < up && curr < left && curr < down && curr < right {
                    low_points.push(((x, y), curr));
                }
            }
        }
        low_points
    }

    fn get_next_coords(
        &self,
        seent: &HashSet<(usize, usize)>,
        x: usize,
        y: usize,
    ) -> Option<(usize, usize)> {
        let next = self.get(x, y)?;

        if !seent.contains(&(x, y)) && *next != 9 {
            Some((x, y))
        } else {
            None
        }
    }

    fn get_product_of_top_three_basins_by_size(&self) -> i32 {
        let mut one = 0;
        let mut two = 0;
        let mut three = 0;
        let mut seent = HashSet::new();
        let low_points = self.get_low_points();
        for ((x,y), _) in low_points {
            let mut stack = vec![(x, y)];
            let mut current_basin_size = 0;
            while !stack.is_empty() {
                let (x, y) = stack.pop().unwrap();

                //  TODO: WHY IS THIS NECESSARY?
                if seent.contains(&(x,y)) {
                    continue;
                }
                current_basin_size += 1;
                seent.insert((x, y));

                //  TODO clean up!
                if x != 0 {
                    let next = self.get_next_coords(&seent, x - 1, y);
                    if let Some(coords) = next {
                        stack.push(coords);
                    }
                };
                if y != 0 {
                    let next = self.get_next_coords(&seent, x, y - 1);
                    if let Some(coords) = next {
                        stack.push(coords);
                    }
                };
                let next = self.get_next_coords(&seent, x + 1, y);
                if let Some(coords) = next {
                    stack.push(coords);
                };

                let next = self.get_next_coords(&seent, x, y + 1);
                if let Some(coords) = next {
                    stack.push(coords);
                };
            }
            
            if current_basin_size > one {
                three = two;
                two = one;
                one = current_basin_size;
            } else if current_basin_size > two {
                three = two;
                two = current_basin_size;
            } else if current_basin_size > three {
                three = current_basin_size;
            }

        }

        one * two * three
    }
}

impl AoCDay for Code {
    fn part1(&self, input: &str, _extra_args: &[String]) -> String {
        let map = Mapping::from_str(input);
        let low_points = map.get_low_points();
        let answer: i32 = low_points.iter().map(|(_, x)| 1 + *x).sum();
        debug_assert_eq!(answer, 591);
        format!("{}", answer) // 591/~2000
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let map = Mapping::from_str(input);
        let answer = map.get_product_of_top_three_basins_by_size();

        debug_assert_eq!(answer, 1113424);
        format!("{:?}", answer) // 1113424/~3000μs
    }
}

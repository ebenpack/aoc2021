use crate::AoCDay;

pub struct Code;

struct Mapping {
    map: Vec<i32>,
    x_size: usize,
    y_size: usize,
}

impl Mapping {
    #[inline(always)]
    fn from_str(input: &str) -> Self {
        let mut map = Vec::with_capacity(100 * 100);
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
    fn set(&mut self, x: usize, y: usize, val: i32) {
        if x < self.x_size && y < self.y_size {
            self.map[x + (self.x_size * y)] = val;
        }
    }
    #[inline(always)]
    fn get_low_points(&self) -> Vec<((usize, usize), i32)> {
        let mut low_points = vec![];
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let curr = self.get(x, y).unwrap();
                let up = if y == 0 {
                    99
                } else {
                    self.get(x, y - 1).unwrap_or(99)
                };
                let left = if x == 0 {
                    99
                } else {
                    self.get(x - 1, y).unwrap_or(99)
                };
                let down = self.get(x, y + 1).unwrap_or(99);
                let right = self.get(x + 1, y).unwrap_or(99);
                if curr < up && curr < left && curr < down && curr < right {
                    low_points.push(((x, y), curr));
                }
            }
        }
        low_points
    }
    #[inline(always)]
    fn get_next_coords(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let next = self.get(x, y)?;
        if next != 9 {
            Some((x, y))
        } else {
            None
        }
    }
    #[inline(always)]
    fn get_product_of_top_three_basins_by_size(mut self) -> i32 {
        // For performance reasons, we're going to mark the visited points in-place,
        // so we'll move the mapping in this method so no one else can use it.
        let mut one = 0;
        let mut two = 0;
        let mut three = 0;
        let low_points = self.get_low_points();
        for ((x, y), _) in low_points {
            let mut stack = vec![(x, y)];
            let mut current_basin_size = 0;
            while !stack.is_empty() {
                let (x, y) = stack.pop().unwrap();

                if let Some(num) = self.get(x, y) {
                    if num == -1 {
                        continue;
                    }
                }
                current_basin_size += 1;
                self.set(x, y, -1);

                if x != 0 {
                    if let Some(coords) = self.get_next_coords(x - 1, y) {
                        stack.push(coords);
                    }
                };
                if y != 0 {
                    if let Some(coords) = self.get_next_coords(x, y - 1) {
                        stack.push(coords);
                    }
                };
                if let Some(coords) = self.get_next_coords(x + 1, y) {
                    stack.push(coords);
                };

                if let Some(coords) = self.get_next_coords(x, y + 1) {
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
        let answer: i32 = Mapping::from_str(input)
            .get_low_points()
            .iter()
            .map(|(_, x)| 1 + *x)
            .sum();
        debug_assert_eq!(answer, 591);
        format!("{}", answer) // 591/~800μs
    }

    fn part2(&self, input: &str, _extra_args: &[String]) -> String {
        let answer = Mapping::from_str(input).get_product_of_top_three_basins_by_size();
        debug_assert_eq!(answer, 1113424);
        format!("{:?}", answer) // 1113424/~1200μs
    }
}

use std::ops::{Add, AddAssign};

#[derive(Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

const DIRS: [Coord; 4] = [
    Coord { x: -1, y: 0 },
    Coord { x: 1, y: 0 },
    Coord { x: 0, y: -1 },
    Coord { x: 0, y: 1 },
];

struct Grid {
    data: Vec<u8>,
    x: usize,
    y: usize,
}

impl Grid {
    fn set(&mut self, c: Coord, e: u8) {
        let i = self.x * c.y as usize + c.x as usize;
        if let Some(num) = self.data.get_mut(i) {
            *num = e;
        }
    }

    fn get(&self, coord: Coord) -> Option<u8> {
        let i = (self.x * coord.y as usize) + coord.x as usize;

        if let Some(num) = self.data.get(i) {
            Some(num.to_owned())
        } else {
            None
        }
    }

    fn is_outside(&self, coord: &Coord) -> bool {
        coord.x < 0
            || coord.x >= self.x.try_into().unwrap()
            || coord.y < 0
            || coord.y >= self.y.try_into().unwrap()
    }

    fn get_viewing_distance(&self, point: Coord, dir: Coord) -> Option<usize> {
        if let Some(height) = self.get(point) {
            let mut pos = Coord { ..point };
            let mut dist = 0;

            while !self.is_outside({
                pos += dir;
                &pos
            }) {
                if let Some(other) = self.get(pos) {
                    if other < height {
                        dist += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            return Some(dist);
        } else {
            None
        }
    }

    fn get_dist_to_edge(&self, pos: Coord, dir: Coord) -> Option<i32> {
        if !self.is_outside(&pos) {
            match dir {
                Coord { x: 0, y: 1 } => Some(self.y as i32 - pos.y - 1),
                Coord { x: 0, y: -1 } => Some(pos.y),
                Coord { x: 1, y: 0 } => Some(self.x as i32 - pos.x - 1),
                Coord { x: -1, y: 0 } => Some(pos.x),
                _ => None,
            }
        } else {
            None
        }
    }

    fn is_visible(&self, pos: Coord) -> bool {
        for dir in DIRS {
            let dist = self.get_viewing_distance(pos, dir).unwrap_or(0);
            let d_edge = self.get_dist_to_edge(pos, dir).unwrap_or(0) as usize;

            if dist == d_edge {
                return true;
            }
        }

        false
    }
}

fn make_grid(input: &str) -> Grid {
    let mut lines = input.trim().lines();
    let mut x = 0;
    let mut y = 0;
    if let Some(line) = lines.next() {
        x = line.trim().len();
        y = lines.count() + 1;
    }

    let mut grid = Grid {
        data: vec![0; x * y],
        x,
        y,
    };

    let tmp = input.trim().replace('\n', "");
    let data = tmp.split("").filter(|s| s != &"").collect::<Vec<&str>>();

    for gx in 0..grid.x {
        for gy in 0..grid.y {
            if let Some(c) = data.get(grid.x * gy + gx) {
                if let Ok(n) = c.parse::<u8>() {
                    grid.set(Coord::new(gx as i32, gy as i32), n);
                }
            }
        }
    }

    grid
}

fn count_visible_cells(grid: &Grid) -> u32 {
    let mut sum = 0;
    for x in 0..grid.x {
        for y in 0..grid.y {
            if grid.is_visible(Coord::new(x as i32, y as i32)) {
                sum += 1;
            }
        }
    }
    sum
}

fn get_max_scenic_score(grid: &Grid) -> usize {
    let mut max_score = 0;

    for x in 0..grid.x {
        for y in 0..grid.y {
            let pos = Coord::new(x as i32, y as i32);
            let mut score = 1;
            for dir in DIRS {
                let dist = grid.get_viewing_distance(pos, dir).unwrap_or(0);

                let d_edge = grid.get_dist_to_edge(pos, dir).unwrap_or(0) as usize;

                // If blocked by tree, include tree in score
                score *= dist + if dist != d_edge {1} else {0};
            }

            max_score = max_score.max(score);
        }
    }

    return max_score;
}

fn main() {
    let input = include_str!("./input.txt");
    let grid = make_grid(input);

    println!("Num visible trees: {}", count_visible_cells(&grid));

    println!("Max scenic score: {}", get_max_scenic_score(&grid));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn test_part_1() {
        let grid = make_grid(TEST_INPUT);

        assert_eq!(count_visible_cells(&grid), 21);
    }

    #[test]
    fn test_part_2() {
        let grid = make_grid(TEST_INPUT);

        assert_eq!(get_max_scenic_score(&grid), 8);
    }
}

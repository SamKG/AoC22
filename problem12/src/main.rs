use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::Read,
    ops,
    path::Path,
};

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<i32>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl From<&String> for Grid {
    fn from(s: &String) -> Self {
        let mut cells = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                let cell_height = match c {
                    'S' => 0,
                    'E' => 'z' as i32 - 'a' as i32,
                    _ => c as i32 - 'a' as i32,
                };

                if c == 'S' {
                    start = (cells.len(), row.len());
                }
                if c == 'E' {
                    end = (cells.len(), row.len());
                }

                row.push(cell_height);
            }
            cells.push(row);
        }
        let height = cells.len();
        let width = cells[0].len();

        Self {
            cells,
            height,
            width,
            start,
            end,
        }
    }
}

impl Grid {
    fn neighbors(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let (i, j) = point;
        let height = self[i][j];

        if i > 0 {
            neighbors.push((i - 1, j));
        }
        if i < self.cells.len() - 1 {
            neighbors.push((i + 1, j));
        }
        if j > 0 {
            neighbors.push((i, j - 1));
        }
        if j < self.cells[0].len() - 1 {
            neighbors.push((i, j + 1));
        }

        neighbors.retain(|&(i, j)| self[i][j] - height <= 1);

        neighbors
    }
}

impl ops::Index<(usize, usize)> for Grid {
    type Output = i32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        &self.cells[i][j]
    }
}

impl ops::Index<usize> for Grid {
    type Output = Vec<i32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

fn min_path(
    grid: &Grid,
    start: (usize, usize),
    end: (usize, usize),
) -> Result<usize, &'static str> {
    let mut queue = BinaryHeap::new();
    let mut min_dists = vec![vec![std::usize::MAX; grid.width]; grid.height];

    queue.push(Reverse((0 as usize, start)));

    while !queue.is_empty() {
        let (dist, current) = queue.pop().unwrap().0;

        if min_dists[current.0][current.1] <= dist {
            continue;
        };

        if current == end {
            return Ok(dist);
        }

        min_dists[current.0][current.1] = dist;

        for &neighbor in grid
            .neighbors(current)
            .iter()
            .filter(|&p| min_dists[p.0][p.1] >= dist + 1)
        {
            queue.push(Reverse((dist + 1, neighbor)));
        }
    }

    Err("No path found")
}

fn solve_part1(input: &String) -> i64 {
    let grid = Grid::from(input);

    min_path(&grid, grid.start, grid.end).unwrap() as i64
}

fn solve_part2(input: &String) -> i64 {
    let grid = Grid::from(input);

    let mut min_dist = std::usize::MAX;

    for i in 0..grid.height {
        for j in 0..grid.width {
            if grid[i][j] == 0 {
                if let Ok(dist) = min_path(&grid, (i, j), grid.end) {
                    if dist < min_dist {
                        min_dist = dist;
                    }
                }
            }
        }
    }

    min_dist as i64
}

fn main() {
    let root_dir = Path::new(file!()).ancestors().nth(2).unwrap();
    let f_path = root_dir.join("input.txt");

    let mut f = File::open(f_path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let ans_1 = solve_part1(&buf);
    println!("Answer 1: {}", ans_1);

    let ans_2 = solve_part2(&buf);
    println!("Answer 2: {}", ans_2);
}

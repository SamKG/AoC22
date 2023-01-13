use std::{fs::File, io::Read};

struct TreeGrid {
    grid: Vec<Vec<u32>>,
}

impl TreeGrid {
    fn is_visible(&self, x: usize, y: usize) -> bool {
        let curr_tree = self.grid[y][x];

        // from north
        if self.grid[0..y].iter().map(|v| v[x]).all(|x| x < curr_tree) {
            return true;
        }

        // from south
        if self.grid[y + 1..self.grid.len()]
            .iter()
            .map(|v| v[x])
            .all(|x| x < curr_tree)
        {
            return true;
        }

        // from west
        if self.grid[y][0..x].iter().all(|&x| x < curr_tree) {
            return true;
        }

        // from east
        if self.grid[y][x + 1..self.grid[y].len()]
            .iter()
            .all(|&x| x < curr_tree)
        {
            return true;
        }

        return false;
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let curr_tree = self.grid[y][x];

        if (x == 0 || x == self.grid[y].len() - 1) && (y == 0 || y == self.grid.len() - 1) {
            return 0;
        }

        let score_north = (0..y)
            .rev()
            .map(|y| self.grid[y][x])
            .take_while(|&x| x < curr_tree)
            .count()
            + 1;

        // from south
        let score_south = (y + 1..self.grid.len())
            .map(|y| self.grid[y][x])
            .take_while(|&x| x < curr_tree)
            .count()
            + 1;

        // from west
        let score_west = (0..x)
            .rev()
            .map(|x| self.grid[y][x])
            .take_while(|&x| x < curr_tree)
            .count()
            + 1;

        // from east
        let score_east = (x + 1..self.grid[y].len())
            .map(|x| self.grid[y][x])
            .take_while(|&x| x < curr_tree)
            .count()
            + 1;

        return score_north.min(y)
            * score_south.min(self.grid.len() - y - 1)
            * score_west.min(x)
            * score_east.min(self.grid[y].len() - x - 1);
    }
}

fn main() {
    let mut f = File::open("problem8/input.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let tree_grid = TreeGrid {
        grid: buf
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect(),
    };

    let mut ans_1 = 0;
    for y in 0..tree_grid.grid.len() {
        for x in 0..tree_grid.grid[y].len() {
            if tree_grid.is_visible(x, y) {
                ans_1 += 1;
            }
        }
    }

    println!("Answer 1: {}", ans_1);

    let mut ans_2 = 0;

    for y in 0..tree_grid.grid.len() {
        for x in 0..tree_grid.grid[y].len() {
            ans_2 = ans_2.max(tree_grid.scenic_score(x, y));
        }
    }

    println!("Answer 2: {}", ans_2);
}

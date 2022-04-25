use std::{fmt::Debug};

use rand::prelude::*;

#[derive(Debug, Clone)]
struct Connection {
    pub index: usize,
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}
impl Connection {
    pub fn new(index: usize, north: bool, south: bool, east: bool, west: bool) -> Self {
        Connection {
            index,
            north,
            south,
            east,
            west,
        }
    }
}

fn main() {
    let (width, height) = (128, 50);

    let things = [
        '┘', '┐', '┌', '└', '┤', '┴', '┬', '├', '─', '│', '┼', ' ', '╷', '╶', '╴', '╵',
    ];

    let connections = vec![
        Connection::new(0, true, false, false, true),
        Connection::new(1, false, true, false, true),
        Connection::new(2, false, true, true, false),
        Connection::new(3, true, false, true, false),
        Connection::new(4, true, true, false, true),
        Connection::new(5, true, false, true, true),
        Connection::new(6, false, true, true, true),
        Connection::new(7, true, true, true, false),
        Connection::new(8, false, false, true, true),
        Connection::new(9, true, true, false, false),
        Connection::new(10, true, true, true, true),
        Connection::new(11, false, false, false, false),
        Connection::new(12, false, true, false, false),
        Connection::new(13, false, false, true, false),
        Connection::new(14, false, false, false, true),
        Connection::new(15, true, false, false, false),
    ];
    let mut grid: Vec<Vec<Vec<Connection>>> = vec![vec![connections.clone(); width]; height];

    let mut rng = thread_rng();

    // Main loop
    while !is_collapsed(&grid) {
        let (y, x) = get_min_enthropy(&grid);

        let mut cell = &mut grid[y][x];

        // Removing edge cases
        *cell = cell
            .clone()
            .iter_mut()
            .filter(|c| {
                !((y == 0) && c.north)
                    && !((y == height - 1) && c.south)
                    && !((x == 0) && c.west)
                    && !((x == width - 1) && c.east)
            })
            .map(|c| c.clone())
            .collect();

        // Randomely choosing an available tile for the selected cell 
        let chosen_value = cell.get(rng.gen_range(0, cell.len())).unwrap().clone();
        *cell = vec![chosen_value.clone()];

        // Propagating the change to adjacent tiles
        if y >= 1 {
            let ny = y - 1;
            if grid[ny][x].len() > 1 {
                grid[ny][x] = grid[ny][x]
                    .iter()
                    .filter(|c| c.south == chosen_value.north)
                    .map(|c| c.clone())
                    .collect();
            }
        }
        let ny = y + 1;
        if ny < height && grid[ny][x].len() > 1 {
            grid[ny][x] = grid[ny][x]
                .iter()
                .filter(|c| c.north == chosen_value.south)
                .map(|c| c.clone())
                .collect();
        }
        let nx = x + 1;
        if nx < width && grid[y][nx].len() > 1 {
            grid[y][nx] = grid[y][nx]
                .iter()
                .filter(|c| c.west == chosen_value.east)
                .map(|c| c.clone())
                .collect();
        }
        if x >= 1 {
            let nx = x - 1;
            if grid[y][nx].len() > 1 {
                grid[y][nx] = grid[y][nx]
                    .iter()
                    .filter(|c| c.east == chosen_value.west)
                    .map(|c| c.clone())
                    .collect();
            }
        }

        render(&grid, &things);
    }
    // TODO: generate a sigle tree fome the bigger path generated
    //       and clean all smaller paths
}

fn render(grid: &Vec<Vec<Vec<Connection>>>, chars: &[char]) {
    let mut out = String::default();
    for i in grid {
        for j in i {
            if j.len() > 1 {
                out += " ";
            } else {
                out += &chars[j[0].index].to_string();
            }
        }
        out += "\n";
    }
    println!("{}", out);
}

fn is_collapsed<T>(grid: &Vec<Vec<Vec<T>>>) -> bool {
    grid.iter().all(|e| e.iter().all(|cell| cell.len() == 1))
}

// Finds the cell with the lowest enthropy (>1) and return it's location if it exists
fn get_min_enthropy<T>(grid: &Vec<Vec<Vec<T>>>) -> (usize, usize) {
    let mut lowests = vec![(0, 0, 0)];

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cell = grid[y][x].len();
            if cell == 1 {
                continue;
            }
            if cell < lowests[0].0 || lowests[0].0 <= 1 {
                lowests = vec![(cell, y, x)];
            } else if cell == lowests[0].0 {
                lowests.push((cell, y, x));
            }
        }
    }

    let mut rng = thread_rng();
    let i = rng.gen_range(0, lowests.len());
    (lowests[i].1, lowests[i].2)
}

#[cfg(test)]
mod tests {
    use crate::{get_min_enthropy, is_collapsed};

    #[test]
    fn test_is_collapsed() {
        let collapsed_grid = vec![vec![vec![0]; 32]; 32];
        let uncollapsed_grid = vec![vec![vec![0; 32]; 32]; 32];
        assert!(is_collapsed(&collapsed_grid));
        assert!(!is_collapsed(&uncollapsed_grid));
    }

    #[test]
    fn test_get_min_enthropy() {
        let mut grid = vec![vec![vec![69; 32]; 32]; 32];
        grid[24][12].pop();
        grid[2][5] = vec![420];
        let res = get_min_enthropy(&grid);
        assert_eq!(res, (24, 12));
    }
}

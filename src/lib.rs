use grid::Grid;

pub mod res;

pub enum EdgeCaseMethod {
    AssumeDead,
    Torodial,
}

pub fn random_grid(width: usize, height: usize) -> Grid<bool> {
    let mut grid = Grid::new(height, width);
    for cell in grid.iter_mut() {
        *cell = rand::random();
    }
    grid
}

pub fn parse_goln(goln: &str) -> Grid<bool> {
    let mut lines = goln.lines();
    let (w, h) = lines
        .next()
        .unwrap()
        .split_once('x')
        .map(map_split_once_to_usize)
        .unwrap();

    let mut grid = Grid::new(h, w);
    lines.for_each(|cell| {
        let (x, y) = cell.split_once(',').map(map_split_once_to_usize).unwrap();
        grid[(y, x)] = true
    });

    grid
}

fn map_split_once_to_usize(tuple: (&str, &str)) -> (usize, usize) {
    (
        tuple.0.parse::<usize>().unwrap(),
        tuple.1.parse::<usize>().unwrap(),
    )
}

pub fn step(grid: &mut Grid<bool>, method: &EdgeCaseMethod) -> bool {
    let mut flips: Vec<(usize, usize)> = Vec::new();

    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let sx = x as isize;
            let sy = y as isize;

            // 0, 1, 2
            // 3,    4
            // 5, 6, 7
            #[rustfmt::skip]
            let live_neighbours = [
                safe_index_grid(grid, method, sx - 1, sy - 1), safe_index_grid(grid, method, sx, sy - 1), safe_index_grid(grid, method, sx + 1, sy - 1),
                safe_index_grid(grid, method, sx - 1, sy),                                                safe_index_grid(grid, method, sx + 1, sy),
                safe_index_grid(grid, method, sx - 1, sy + 1), safe_index_grid(grid, method, sx, sy + 1), safe_index_grid(grid, method, sx + 1, sy + 1),
            ].iter().filter(|cell| **cell).count();

            if grid[(y, x)] {
                if !(2..=3).contains(&live_neighbours) {
                    flips.push((x, y));
                }
            } else if live_neighbours == 3 {
                flips.push((x, y));
            }
        }
    }

    if flips.is_empty() {
        false
    } else {
        for flip in flips {
            grid[(flip.1, flip.0)] = !grid[(flip.1, flip.0)]
        }
        true
    }
}

fn safe_index_grid(grid: &Grid<bool>, method: &EdgeCaseMethod, x: isize, y: isize) -> bool {
    match method {
        EdgeCaseMethod::AssumeDead => {
            if x < 0 || y < 0 || (x as usize) >= grid.cols() || (y as usize) >= grid.rows() {
                false
            } else {
                grid[(y as usize, x as usize)]
            }
        }
        EdgeCaseMethod::Torodial => {
            let cols = grid.cols() as isize;
            let rows = grid.rows() as isize;

            let x = if x >= cols {
                x - cols
            } else if x < 0 {
                cols + x
            } else {
                x
            };
            let y = if y >= rows {
                y - rows
            } else if y < 0 {
                rows + y
            } else {
                y
            };

            grid[(y as usize, x as usize)]
        }
    }
}

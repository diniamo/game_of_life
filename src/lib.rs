use grid::Grid;

pub mod res;

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
    lines.next().unwrap().split(';').for_each(|cell| {
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

pub fn step(grid: Grid<bool>) -> Grid<bool> {
    let mut new_grid = Grid::new(grid.rows(), grid.cols());

    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let sx = x as i64;
            let sy = y as i64;

            // 0, 1, 2
            // 3,    4
            // 5, 6, 7
            #[rustfmt::skip]
            let neighbours = [
                safe_index_grid(&grid, sx - 1, sy - 1), safe_index_grid(&grid, sx, sy - 1), safe_index_grid(&grid, sx + 1, sy - 1),
                safe_index_grid(&grid, sx - 1, sy),                                         safe_index_grid(&grid, sx + 1, sy),
                safe_index_grid(&grid, sx - 1, sy + 1), safe_index_grid(&grid, sx, sy + 1), safe_index_grid(&grid, sx + 1, sy + 1),
            ];

            let live_neighbours = neighbours
                .iter()
                .filter(|cell| cell.unwrap_or(false))
                .count();

            if grid[(y, x)] && !(2..=3).contains(&live_neighbours) {
                new_grid[(y, x)] = false;
            } else if live_neighbours == 3 {
                new_grid[(y, x)] = true;
            } else {
                new_grid[(y, x)] = grid[(y, x)];
            }
        }
    }

    new_grid
}

fn safe_index_grid<T: Copy>(grid: &Grid<T>, x: i64, y: i64) -> Option<T> {
    if x < 0 || y < 0 || (x as usize) >= grid.cols() || (y as usize) >= grid.rows() {
        None
    } else {
        Some(grid[(y as usize, x as usize)])
    }
}

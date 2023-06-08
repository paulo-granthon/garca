use std::fs;
use std::io::{self, Read};
use std::path::Path;
use rand::Rng;

// Define the dimensions of the grid
const GRID_WIDTH: usize = 240;
const GRID_HEIGHT: usize = 32;

// Define the scaling of the grid
const SVG_CELL_SCALE: usize = 4;

// Initial probability of generating an '1' cell when reseting the grid (0 ~ 100)
const RAND_POPULATE_CHANCE: usize = 30;

// Custom struct for the grid
#[derive(Debug)]
struct Grid {
    cells: [[u8; GRID_WIDTH]; GRID_HEIGHT],
}

impl Grid {
    // Randomize the grid with binary cell values
    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..GRID_HEIGHT {
            for j in 0..GRID_WIDTH {
                self.cells[i][j] = if rng.gen_range(0..101) < RAND_POPULATE_CHANCE { 1 } else { 0 };
            }
        }
    }

    // Save the grid state to a file
    fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let grid_string = self.to_string();
        fs::write(file_path, grid_string)
    }

    // Convert the grid to a string representation
    fn to_string(&self) -> String {
        let mut grid_string = String::new();

        for row in self.cells.iter() {
            let row_string = row.iter().map(|cell| cell.to_string()).collect::<Vec<_>>().join(",");
            grid_string.push_str(&row_string);
            grid_string.push('\n');
        }

        grid_string
    }

    // Generate an SVG string representation of the grid
    fn to_svg_string(&self) -> String {
        let mut svg_string = String::new();

        // SVG header
        svg_string.push_str(
            format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
                GRID_WIDTH * SVG_CELL_SCALE,
                GRID_HEIGHT * SVG_CELL_SCALE
            ).as_str()
        );

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let x = j * SVG_CELL_SCALE;
                let y = i * SVG_CELL_SCALE;

                // SVG rectangle representing the cell
                svg_string.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
                    x, y, SVG_CELL_SCALE, SVG_CELL_SCALE, if *cell == 1 { "white" } else { "black" }
                ));
            }
        }

        // SVG footer
        svg_string.push_str("</svg>");

        svg_string
    }
}

// Define the rule for updating cell states
fn update_cell_state(grid: &mut Grid) -> bool {
    let mut new_grid = Grid { cells: [[0; GRID_WIDTH]; GRID_HEIGHT] };
    let mut changed = false;

    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            let cell = grid.cells[i][j];
            let neighbors = count_neighbors(&grid.cells, i, j);

            if cell == 1 && neighbors < 2 {
                new_grid.cells[i][j] = 0; // Cell dies due to underpopulation
                changed = true;
            } else if cell == 1 && (neighbors == 2 || neighbors == 3) {
                new_grid.cells[i][j] = 1; // Cell survives to the next generation
            } else if cell == 1 && neighbors > 3 {
                new_grid.cells[i][j] = 0; // Cell dies due to overpopulation
                changed = true;
            } else if cell == 0 && neighbors == 3 {
                new_grid.cells[i][j] = 1; // Cell is born due to reproduction
                changed = true;
            }
        }
    }

    grid.cells = new_grid.cells;
    changed
}

// Count the number of live neighbors for a given cell
fn count_neighbors(grid: &[[u8; GRID_WIDTH]; GRID_HEIGHT], row: usize, col: usize) -> u8 {
    let mut count = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue; // Skip the current cell
            }

            let neighbor_row = (row as isize + i + GRID_HEIGHT as isize) % GRID_HEIGHT as isize;
            let neighbor_col = (col as isize + j + GRID_WIDTH as isize) % GRID_WIDTH as isize;

            let neighbor_cell = grid[neighbor_row as usize][neighbor_col as usize];
            count += neighbor_cell;
        }
    }

    count
}

fn main() -> io::Result<()> {
    let file_path = "grid.txt";
    let mut grid = read_grid_from_file(file_path)?;

    let changed = update_cell_state(&mut grid);

    if !changed {
        grid.randomize();
        grid.save_to_file(file_path)?;
    } else {
        grid.save_to_file(file_path)?;
    }

    let svg_string = grid.to_svg_string();
    fs::write("grid.svg", svg_string)?;

    Ok(())
}

// Read the grid from a file or generate a random grid if the file doesn't exist
fn read_grid_from_file(file_path: &str) -> io::Result<Grid> {
    if Path::new(file_path).exists() {
        read_grid_existing(file_path)
    } else {
        let mut grid = Grid { cells: [[0; GRID_WIDTH]; GRID_HEIGHT] };
        grid.randomize();
        grid.save_to_file(file_path)?;
        Ok(grid)
    }
}

// Read the grid from an existing file
fn read_grid_existing(file_path: &str) -> io::Result<Grid> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut grid = Grid { cells: [[0; GRID_WIDTH]; GRID_HEIGHT] };
    let lines: Vec<&str> = contents.trim().split('\n').collect();

    for (i, line) in lines.iter().enumerate().take(GRID_HEIGHT) {
        let values: Vec<u8> = line
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        if values.len() >= GRID_WIDTH {
            grid.cells[i].copy_from_slice(&values[..GRID_WIDTH]);
        } else {
            grid.cells[i][..values.len()].copy_from_slice(&values);
        }
    }

    Ok(grid)
}

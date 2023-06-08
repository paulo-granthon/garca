use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use rand::Rng;

// Define the dimensions of the grid
const GRID_WIDTH: usize = 240;
const GRID_HEIGHT: usize = 32;

// Define the scaling of the grid
const SVG_CELL_SCALE: usize = 4;

// Initial probability of generating an '1' cell when reseting the grid (0 ~ 100)
const RAND_POPULATE_CHANCE: usize = 30;

// The name of the folder where the state history of the Grid will be saved 
const HISTORY_FOLDER: &'static str = "history";

#[derive(Debug, Clone, Copy)]
pub struct State {
    cells: [[u8; GRID_WIDTH]; GRID_HEIGHT]
}

impl State {
    pub fn iter(&self) -> impl Iterator<Item = [u8; GRID_WIDTH]> {
        self.cells.into_iter()
    }

    // Save the grid state to a file
    fn save_to_file(&self, file_path: PathBuf) -> io::Result<()> {
        let grid_string = self.to_string();
        fs::write(file_path, grid_string)
    }

    // Convert the grid to a string representation
    fn to_string(&self) -> String {
        let mut grid_string = String::new();

        for row in self.iter() {
            let row_string = row.iter().map(|cell| cell.to_string()).collect::<Vec<_>>().join(",");
            grid_string.push_str(&row_string);
            grid_string.push('\n');
        }

        grid_string
    }

    // Generate an SVG string representation of the grid state
    fn to_svg_string(&self) -> String {
        let mut svg_string = String::new();

        // SVG header
        svg_string.push_str(
            format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
                GRID_WIDTH * SVG_CELL_SCALE,
                GRID_HEIGHT * SVG_CELL_SCALE
            )
            .as_str(),
        );

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let x = j * SVG_CELL_SCALE;
                let y = i * SVG_CELL_SCALE;

                // SVG rectangle representing the cell
                svg_string.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />\n",
                    x,
                    y,
                    SVG_CELL_SCALE,
                    SVG_CELL_SCALE,
                    if *cell == 1 { "black" } else { "white" }
                ));
            }
        }

        // SVG footer
        svg_string.push_str("</svg>");

        svg_string
    }

}

impl Default for State {
    fn default() -> Self {
        Self {
            cells: [[u8::default(); GRID_WIDTH]; GRID_HEIGHT],
        }
    }
}

impl std::ops::Index<usize> for State {
    type Output = [u8; GRID_WIDTH];

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl std::ops::IndexMut<usize> for State {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl From<State> for [[u8; GRID_WIDTH]; GRID_HEIGHT] {
    fn from(state: State) -> Self {
        state.cells
    }
}

#[derive(Default)]
pub struct Grid {
    state: State,
    history: Vec<State>,
}

impl Grid {

    // Randomize the grid with binary cell values
    fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..GRID_HEIGHT {
            for j in 0..GRID_WIDTH {
                self.state[i][j] = if rng.gen_range(0..101) < RAND_POPULATE_CHANCE { 1 } else { 0 };
            }
        }
    }

    // Save the history files in the specified folder
    fn save_state_history(&self) -> io::Result<()> {
        for (index, state) in self.history.iter().enumerate() {
            let file_name = format!("state_{}.txt", index);

            state.save_to_file(Path::new(&HISTORY_FOLDER).join(file_name))?;
        }

        Ok(())
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

        for (i, row) in self.state.iter().enumerate() {
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
    let mut new_grid = State::default();
    let mut changed = false;

    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            let cell = grid.state[i][j];
            let neighbors = count_neighbors(&grid.state.cells, i, j);

            if cell == 1 && neighbors < 2 {
                new_grid[i][j] = 0; // Cell dies due to underpopulation
                changed = true;
            } else if cell == 1 && (neighbors == 2 || neighbors == 3) {
                new_grid[i][j] = 1; // Cell survives to the next generation
            } else if cell == 1 && neighbors > 3 {
                new_grid[i][j] = 0; // Cell dies due to overpopulation
                changed = true;
            } else if cell == 0 && neighbors == 3 {
                new_grid[i][j] = 1; // Cell is born due to reproduction
                changed = true;
            }
        }
    }

    grid.history.push(grid.state);
    grid.state = new_grid;
    changed
}

// Count the number of live neighbors for a given cell
fn count_neighbors(grid: &[[u8; GRID_WIDTH]; GRID_HEIGHT], row: usize, col: usize) -> u8 {
    let mut count = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 { continue; } // Skip the current cell

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

    if !changed { grid.randomize(); }

    grid.save_state_history()?;

    // Generate animated SVG from history
    generate_animated_svg()?;

    Ok(())
}

// Read the grid from a file or generate a random grid if the file doesn't exist
fn read_grid_from_file(file_path: &str) -> io::Result<Grid> {
    if Path::new(file_path).exists() {
        read_grid_existing(file_path)
    } else {
        let mut grid = Grid::default();
        grid.randomize();
        Ok(grid)
    }
}

// Read the grid from an existing file
fn read_grid_existing(file_path: &str) -> io::Result<Grid> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut grid = Grid::default();
    let lines: Vec<&str> = contents.trim().split('\n').collect();

    for (i, line) in lines.iter().enumerate().take(GRID_HEIGHT) {
        let values: Vec<u8> = line
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        if values.len() >= GRID_WIDTH {
            grid.state[i].copy_from_slice(&values[..GRID_WIDTH]);
        } else {
            grid.state[i][..values.len()].copy_from_slice(&values);
        }
    }

    Ok(grid)
}

fn parse_state_from_string(grid_string: &str) -> io::Result<State> {
    let mut cells: [[u8; GRID_WIDTH]; GRID_HEIGHT] = [[0; GRID_WIDTH]; GRID_HEIGHT];

    let lines: Vec<&str> = grid_string.trim().lines().collect();
    if lines.len() != GRID_HEIGHT {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid grid height"));
    }

    for (i, line) in lines.iter().enumerate() {
        let row: Vec<u8> = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        if row.len() != GRID_WIDTH {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid grid width"));
        }

        for (j, cell) in row.iter().enumerate() {
            cells[i][j] = *cell;
        }
    }

    Ok(State { cells })
}

fn generate_animated_svg() -> io::Result<()> {
    let history_files = fs::read_dir(HISTORY_FOLDER)?;

    let mut frames = String::new();

    for entry in history_files {
        let file = entry?;

        let contents = fs::read_to_string(file.path())?;
        let state = parse_state_from_string(&contents)?;

        let frame = state.to_svg_string();
        let animation_duration = 500; // milliseconds

        frames.push_str(&format!(
            r#"<set attributeName="visibility" to="visible" begin="{}ms" dur="{}ms" fill="freeze" />{}"#,
            animation_duration,
            animation_duration,
            frame
        ));
    }

    let svg_string = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">
            <rect x="0" y="0" width="100%" height="100%" fill="white" />
            <g visibility="hidden">{}</g>
        </svg>"#,
        GRID_WIDTH * 10,
        GRID_HEIGHT * 10,
        frames
    );

    fs::write("animation.svg", svg_string)?;

    Ok(())
}

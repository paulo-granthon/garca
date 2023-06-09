use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use rand::Rng;

// Define the dimensions of the grid
// const GRID_WIDTH: usize = 240;
// const GRID_HEIGHT: usize = 32;
const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 10;

// Define the scaling of the grid
const SVG_CELL_SCALE: usize = 16;

// Initial probability of generating an '1' cell when reseting the grid (0 ~ 100)
const RAND_POPULATE_CHANCE: usize = 30;

// The name of the folder where the state history of the Grid will be saved 
const HISTORY_FOLDER: &'static str = "history";

// The duration of each frame of the .svg animation in milliseconds 
const SVG_FRAME_DURATION_MS: usize = 200;

#[derive(Debug, Clone, Copy)]
pub struct State {
    cells: [[u8; GRID_WIDTH]; GRID_HEIGHT]
}

impl State {

    fn random () -> Self {
        let mut rng = rand::thread_rng();

        let mut cells = [[0; GRID_WIDTH]; GRID_HEIGHT];

        for i in 0..GRID_HEIGHT {
            for j in 0..GRID_WIDTH {
                cells[i][j] = if rng.gen_range(0..101) < RAND_POPULATE_CHANCE { 1 } else { 0 };
            }
        }

        Self {
            cells
        }
    }

    fn iter(&self) -> impl Iterator<Item = [u8; GRID_WIDTH]> {
        self.cells.into_iter()
    }

    // Save the grid state to a file
    fn save_to_file(&self, file_path: PathBuf) -> io::Result<String> {
        let state_string = self.to_string();
        let mut file = fs::File::create(&file_path)?;
        file.write_all(state_string.as_bytes())?;
        // file.flush()?;
        Ok(state_string)
    }

    // Convert the grid to a string representation
    fn to_string(&self) -> String {
        let mut grid_string = String::new();

        for row in self.iter() {
            let row_string = row.iter().map(|cell| cell.to_string()).collect::<Vec<_>>().join("");
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

        let rows = self.cells.len();

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let x = j * SVG_CELL_SCALE;
                let y = i * SVG_CELL_SCALE;

                // SVG rectangle representing the cell
                svg_string.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />{}",
                    x,
                    y,
                    SVG_CELL_SCALE,
                    SVG_CELL_SCALE,
                    if *cell == 1 { "black" } else { "white" },
                    if i < rows - 1 { "\n" } else { "" }
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
    history: Vec<State>,
}

impl Grid {

    fn new (history: Vec<State>) -> Self {
        Self {
            history
        }
    }

    // Randomize the grid with binary cell values
    fn random() -> Self {
        Self {
            history: vec![State::random()]
        }
    }

    // Save the history files in the specified folder
    fn save_state_history(&self) -> io::Result<Vec<String>> {
        delete_all_files_in_directory(HISTORY_FOLDER)?;

        let mut state_strings = vec![];

        for (index, state) in self.history.iter().enumerate() {            
            state_strings.push(
                state.save_to_file(
                    PathBuf::from(&HISTORY_FOLDER).join(format!("state_{}.txt", index))
                )?
            );
        }

        Ok(state_strings)
    }
}

// Define the rule for updating cell states
fn update_cell_state(grid: &mut Grid) -> bool {
    let mut changed = false;
    
    if grid.history.len() < 1 { return false; }

    let mut new_state : State = grid.history[0].clone();

    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            let cell = new_state[i][j];
            let neighbors = count_neighbors(&new_state, i, j);

            if cell == 1 && neighbors < 2 {
                new_state[i][j] = 0; // Cell dies due to underpopulation
                changed = true;
            } else if cell == 1 && (neighbors == 2 || neighbors == 3) {
                new_state[i][j] = 1; // Cell survives to the next generation
            } else if cell == 1 && neighbors > 3 {
                new_state[i][j] = 0; // Cell dies due to overpopulation
                changed = true;
            } else if cell == 0 && neighbors == 3 {
                new_state[i][j] = 1; // Cell is born due to reproduction
                changed = true;
            }
        }
    }

    grid.history.push(new_state);
    changed
}

// Count the number of live neighbors for a given cell
fn count_neighbors(state: &State, row: usize, col: usize) -> u8 {
    let mut count = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 { continue; } // Skip the current cell

            let neighbor_row = (row as isize + i + GRID_HEIGHT as isize) % GRID_HEIGHT as isize;
            let neighbor_col = (col as isize + j + GRID_WIDTH as isize) % GRID_WIDTH as isize;

            let neighbor_cell = state.cells[neighbor_row as usize][neighbor_col as usize];
            count += neighbor_cell;
        }
    }

    count
}

fn main() -> io::Result<()> {

    let mut grid = read_grid_from_folder(HISTORY_FOLDER)?;

    let changed = update_cell_state(&mut grid);

    if !changed { grid = Grid::random(); }

    grid.save_state_history()?;

    // Generate animated SVG from history
    generate_animated_svg(grid.history)?;

    Ok(())
}

// Read the grid from a file or generate a random grid if the file doesn't exist
fn read_grid_from_folder(file_path: &str) -> io::Result<Grid> {
    if !Path::new(file_path).exists() {

        let mut states: Vec<State> = vec![];

        for entry in fs::read_dir(HISTORY_FOLDER)? {
            let file = entry?;
            dbg!("read_grid_from_folder | loading file: {}", file.path().to_string_lossy());
            states.push(read_state_from_file(file.path())?);
        }

        if states.len() > 0{
            states.reverse();
            return Ok(Grid::new(states));
        }
    }

    Ok(Grid::random())
}

// Read the grid from an existing file
fn read_state_from_file(file_path: PathBuf) -> io::Result<State> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut state = State::default();
    let lines: Vec<&str> = contents.trim().split('\n').collect();

    for (i, line) in lines.iter().enumerate().take(GRID_HEIGHT) {
        let values: Vec<u8> = line
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        if values.len() >= GRID_WIDTH {
            state[i].copy_from_slice(&values[..GRID_WIDTH]);
        } else {
            state[i][..values.len()].copy_from_slice(&values);
        }
    }

    Ok(state)
}

// fn parse_state_from_string(grid_string: &str) -> io::Result<State> {
//     let mut cells: [[u8; GRID_WIDTH]; GRID_HEIGHT] = [[0; GRID_WIDTH]; GRID_HEIGHT];

//     let lines: Vec<&str> = grid_string.trim().lines().collect();
//     if lines.len() != GRID_HEIGHT {
//         return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid grid height"));
//     }

//     for (i, line) in lines.iter().enumerate() {
//         let row: Vec<u8> = line
//             .trim()
//             .chars()
//             .filter_map(|c| c.to_digit(2).map(|d| d as u8))
//             // .map(|c| c.to_digit(2).expect("Invalid line on parse_state_from_string") as u8)
//             .collect();

//         if row.len() != GRID_WIDTH {
//             return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid grid width"));
//         }

//         for (j, cell) in row.iter().enumerate() {
//             cells[i][j] = *cell;
//         }
//     }

//     Ok(State { cells })
// }

fn generate_animated_svg(states: Vec<State>) -> io::Result<()> {

    let mut frames = String::new();

    for (i, state) in states.iter().enumerate() {

        let frame = state.to_svg_string();
        let frame_star = i * SVG_FRAME_DURATION_MS;
        let frame_end = (i + 1) * SVG_FRAME_DURATION_MS;

        frames.push_str(&format!(
            r#"<set attributeName="visibility" to="visible" begin="{}ms" dur="{}ms" fill="freeze" />{}"#,
            frame_star,
            frame_end,
            frame
        ));
    }

    let svg_string = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">
            <rect x="0" y="0" width="100%" height="100%" fill="black" />
            <g visibility="hidden">{}</g>
        </svg>"#,
        GRID_WIDTH * SVG_CELL_SCALE,
        GRID_HEIGHT * SVG_CELL_SCALE,
        frames
    );

    fs::write("animation.svg", svg_string)?;

    Ok(())
}

fn delete_all_files_in_directory(dir_path: &str) -> io::Result<()> {
    // Read the directory
    let dir_entries = fs::read_dir(dir_path)?;

    // Iterate over the directory entries
    for entry in dir_entries {
        if let Ok(entry) = entry {
            // Check if the entry is a file
            if entry.file_type()?.is_file() {
                // Delete the file
                fs::remove_file(entry.path())?;
            }
        }
    }

    Ok(())
}

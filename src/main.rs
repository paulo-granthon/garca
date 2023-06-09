use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use rand::Rng;

// Initial probability of generating an '1' cell when reseting the grid (0 ~ 100)
const RAND_POPULATE_CHANCE: usize = 20;

// Define the dimensions of the grid
const GRID_WIDTH: usize = 240;
const GRID_HEIGHT: usize = 32;

// The number of updates per run
const UPDATES_PER_RUN: usize = 4;

// Define the scaling of the grid
const SVG_CELL_SCALE: usize = 4;

// The duration of each frame of the .svg animation in milliseconds 
const SVG_FRAME_DURATION_MS: usize = 300;

// The delay before the animation starts
const SVG_START_DELAY_MS: usize = 500;


// The name of the folder where the state history of the Grid will be saved 
const HISTORY_FOLDER: &'static str = "history";


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
        // svg_string.push_str(
        //     format!(
        //         "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
        //         GRID_WIDTH * SVG_CELL_SCALE,
        //         GRID_HEIGHT * SVG_CELL_SCALE
        //     )
        //     .as_str(),
        // );
        svg_string.push_str("\n\t\t<g>");

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let x = j * SVG_CELL_SCALE;
                let y = i * SVG_CELL_SCALE;

                // SVG rectangle representing the cell
                svg_string.push_str(&format!(
                    "\n\t\t\t<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />",
                    x,
                    y,
                    SVG_CELL_SCALE,
                    SVG_CELL_SCALE,
                    if *cell == 1 { "white" } else { "black" }
                ));
            }
        }

        // SVG footer
        svg_string.push_str("\n\t\t</g>");

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

    fn current_state (&self) -> Option<&State> {
        self.history.last()
    }

    // Save the history files in the specified folder
    fn save_state_history(&self) -> io::Result<Vec<String>> {
        delete_all_files_in_directory(HISTORY_FOLDER)?;

        // println!("save_state_history | initializing");

        let mut state_strings = vec![];

        for (i, state) in self.history.iter().enumerate() {
            let index = format!("{:08}", i);

            println!("save_state_history | creating file [{}]: {:?}", index, PathBuf::from(&HISTORY_FOLDER).join(format!("state_{}.txt", index)));

            state_strings.push(
                state.save_to_file(
                    PathBuf::from(&HISTORY_FOLDER).join(format!("state_{}.txt", index))
                )?
            );
        }

        // println!("save_state_history | {} files saved", state_strings.len());

        Ok(state_strings)
    }
}

// Define the rule for updating cell states
fn update_cell_state(grid: &mut Grid) -> bool {
    // println!("update_cell_state | initializing update");
    let mut changed = false;

    match grid.current_state() {
        Some(previous_state) => {
            let mut new_state : State = previous_state.clone();

            // println!("update_cell_state | new_state cloned from grid.current_state()");
        
            for i in 0..GRID_HEIGHT {
                for j in 0..GRID_WIDTH {
                    let cell = new_state[i][j];
                    let neighbors = count_neighbors(&previous_state, i, j);
        
                    let previous_value = new_state[i][j];
                    let mut new_value = previous_value;
        
                    // if cell == 1 { new_value = 0 }
                    // else if neighbors == 1 { new_value = 1 }
        
                    if cell == 1 && neighbors < 2 {
                        new_value = 0; // Cell dies due to underpopulation
                    }

                    else if cell == 1 && (neighbors == 2 || neighbors == 3) {
                        new_value = 1; // Cell survives to the next generation
                    }

                    else if cell == 1 && neighbors > 3 {
                        new_value = 0; // Cell dies due to overpopulation
                    }

                    else if cell == 0 && neighbors == 3 {
                        new_value = 1; // Cell is born due to reproduction
                    }
        
                    if new_value != previous_value {
                        // println!("update_cell_state | detected change");
                        changed = true;
                    }
        
                    new_state[i][j] = new_value;
                }
            }
        
            grid.history.push(new_state);
        
            // println!("update_cell_state | new_state pushed, now we have {} states in the history!!", grid.history.len());        
        },
        None => {}
    }
    changed
}

// Count the number of live neighbors for a given cell
fn count_neighbors(state: &State, row: usize, col: usize) -> u8 {
    let mut count = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 { continue; } // Skip the current cell
            let ni = ((row as isize + i + GRID_HEIGHT as isize) % GRID_HEIGHT as isize) as usize;
            let nj = ((col as isize + j + GRID_WIDTH as isize) % GRID_WIDTH as isize) as usize;
            count += state.cells[ni][nj];
        }
    }

    count
}

fn main() -> io::Result<()> {

    let mut grid = read_grid_from_folder(HISTORY_FOLDER)?;

    for _ in 0..UPDATES_PER_RUN {
    
        if update_cell_state(&mut grid) { continue }

        println!("grid didn't change in this update");
        grid = Grid::random();
    }

    grid.save_state_history()?;

    // Generate animated SVG from history
    generate_animated_svg(grid.history)?;

    Ok(())
}

// Read the grid from a file or generate a random grid if the file doesn't exist
fn read_grid_from_folder(file_path: &str) -> io::Result<Grid> {
    // println!("read_grid_from_folder | Starting read :)");
    dbg!(Path::new(file_path));
    dbg!(Path::new(file_path).exists());
    if Path::new(file_path).exists() {
        // println!("read_grid_from_folder | Path exists :o");

        let mut states: Vec<State> = vec![];

        for entry in fs::read_dir(HISTORY_FOLDER)? {
            let file = entry?;
            println!("read_grid_from_folder | loading file: {} :D", file.path().to_string_lossy());
            states.push(read_state_from_file(file.path())?);
        }

        if states.len() > 0 {
            // println!("read_grid_from_folder | We found files yay! :)");
            return Ok(Grid::new(states));
        }
    }

    // println!("read_grid_from_folder | Returning random! \\o/");
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

fn generate_animated_svg(states: Vec<State>) -> io::Result<()> {
    let mut frames = String::new();

    for (i, state) in states.iter().enumerate() {
        frames.push_str(&format!(
            "\n\t<g opacity=\"{}\">\n\t\t<animate attributeName=\"opacity\" from=\"0\" to=\"1\" begin=\"{}ms\" dur=\"{}ms\" fill=\"freeze\" />",
            1 - std::cmp::min(i, 1),
            SVG_START_DELAY_MS + (i * SVG_FRAME_DURATION_MS),
            SVG_FRAME_DURATION_MS
        ));
        if i < states.len() - 1 {
            frames.push_str(&format!(
                "\n\t\t<animate attributeName=\"opacity\" from=\"1\" to=\"0\" begin=\"{}ms\" dur=\"{}ms\" fill=\"freeze\" />",
                SVG_START_DELAY_MS + ((i + 1) * SVG_FRAME_DURATION_MS),
                SVG_FRAME_DURATION_MS
            ));    
        }
        frames.push_str(&format!("{}\n\t</g>", state.to_svg_string()));
    }

    let svg_string = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">{}\n</svg>",
        GRID_WIDTH * SVG_CELL_SCALE,
        GRID_HEIGHT * SVG_CELL_SCALE,
        frames
    );

    fs::write("main.svg", svg_string)?;

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

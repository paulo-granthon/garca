mod data; use data::*;
mod crud; use crud::*;

// file types for state crud and output
const READ_AND_WRITE: Ext = Ext::TXT;
const RENDER: RenderExt = RenderExt::SVG;

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
        
            grid.push_history(new_state);
        
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
            count += state.cell_at(ni, nj);
        }
    }

    count
}

fn main() -> std::io::Result<()> {
    let crud: Crud = Crud::new(READ_AND_WRITE, READ_AND_WRITE, RENDER);

    let mut grid = crud.reader().read_grid(HISTORY_FOLDER)?;

    for _ in 0..UPDATES_PER_RUN {
    
        if update_cell_state(&mut grid) { continue }

        println!("grid didn't change in this update");
        grid = Grid::random();
    }

    crud.writer().write_grid(&grid)?;

    // Generate animated SVG from history
    crud.renderer().render_grid(&grid)?;

    Ok(())
}
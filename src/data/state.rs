#[derive(Debug, Clone, Copy)]
pub struct State {
    cells: [[u8; crate::GRID_WIDTH]; crate::GRID_HEIGHT]
}

impl State {

    pub fn random () -> Self {
        let mut rng = rand::thread_rng();

        let mut cells = [[0; crate::GRID_WIDTH]; crate::GRID_HEIGHT];

        for i in 0..crate::GRID_HEIGHT {
            for j in 0..crate::GRID_WIDTH {
                cells[i][j] = if rand::Rng::gen_range(&mut rng, 0..101) < crate::RAND_POPULATE_CHANCE { 1 } else { 0 };
            }
        }

        Self {
            cells
        }
    }

    pub fn cell_at (&self, y: usize, x: usize) -> u8 {
        self.cells[y][x]
    }

    pub fn get_cells (&self) -> [[u8; crate::GRID_WIDTH]; crate::GRID_HEIGHT] {
        self.cells
    }

    pub fn iter(&self) -> impl Iterator<Item = [u8; crate::GRID_WIDTH]> {
        self.cells.into_iter()
    }

}

impl Default for State {
    fn default() -> Self {
        Self {
            cells: [[u8::default(); crate::GRID_WIDTH]; crate::GRID_HEIGHT],
        }
    }
}

impl std::ops::Index<usize> for State {
    type Output = [u8; crate::GRID_WIDTH];

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl std::ops::IndexMut<usize> for State {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl From<State> for [[u8; crate::GRID_WIDTH]; crate::GRID_HEIGHT] {
    fn from(state: State) -> Self {
        state.cells
    }
}


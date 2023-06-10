use super::State;

#[derive(Default)]
pub struct Grid {
    history: Vec<State>,
}

impl Grid {

    pub fn new (history: Vec<State>) -> Self {
        Self {
            history
        }
    }

    // Randomize the grid with binary cell values
    pub fn random() -> Self {
        Self {
            history: vec![State::random()]
        }
    }

    pub fn current_state (&self) -> Option<&State> {
        self.history.last()
    }

    pub fn push_history (&mut self, state: State) {
        self.history.push(state)
    }

    pub fn get_history (&self) -> &Vec<State> {
        &self.history
    }

}

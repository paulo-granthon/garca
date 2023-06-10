pub(crate) struct TxtReader {}

impl super::Reader for TxtReader {

    // Read the grid from a file or generate a random grid if the file doesn't exist
    fn read_grid (&self, file_path: &str) -> std::io::Result<crate::data::Grid> {
        if std::path::Path::new(file_path).exists() {
            let mut states: Vec<crate::data::State> = vec![];
            for file in {
                let mut files = std::fs::read_dir(crate::HISTORY_FOLDER)?
                .map(|entry| entry.unwrap())
                .collect::<Vec<std::fs::DirEntry>>();
                files.sort_by(|a, b| {
                    let a_name = a.file_name();
                    let b_name = b.file_name();
                    a_name.cmp(&b_name)
                });
                files
            } {
                states.push(self.read_state(file.path())?);
            }
            if states.len() > 0 {
                return Ok(crate::data::Grid::new(states));
            }
        }
        Ok(crate::data::Grid::random())
    }

    fn read_state (&self, file_path: std::path::PathBuf) -> std::io::Result<crate::data::State> {
        let mut file = std::fs::File::open(file_path)?;
        let mut contents = String::new();
        std::io::Read::read_to_string(&mut file, &mut contents)?;

        let mut state = crate::data::State::default();
        let lines: Vec<&str> = contents.trim().split('\n').collect();

        for (i, line) in lines.iter().enumerate().take(crate::GRID_HEIGHT) {
            let values: Vec<u8> = line
                .trim()
                .split(',')
                .map(|s| s.parse().unwrap_or(0))
                .collect();
            if values.len() >= crate::GRID_WIDTH {
                state[i].copy_from_slice(&values[..crate::GRID_WIDTH]);
            } else {
                state[i][..values.len()].copy_from_slice(&values);
            }
        }
        Ok(state)
    }
}
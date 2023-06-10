pub(crate) struct TxtReader {}

impl super::Reader for TxtReader {

    // Read the grid from a file or generate a random grid if the file doesn't exist
    fn read_grid (file_path: &str) -> std::io::Result<crate::data::Grid> {
        // println!("read_grid_from_folder | Starting read :)");
        dbg!(std::path::Path::new(file_path));
        dbg!(std::path::Path::new(file_path).exists());
        if std::path::Path::new(file_path).exists() {
            // println!("read_grid_from_folder | Path exists :o");

            let mut states: Vec<crate::data::State> = vec![];

            for entry in std::fs::read_dir(crate::HISTORY_FOLDER)? {
                let file = entry?;
                println!("read_grid_from_folder | loading file: {} :D", file.path().to_string_lossy());
                states.push(TxtReader::read_state(file.path())?);
            }

            if states.len() > 0 {
                // println!("read_grid_from_folder | We found files yay! :)");
                return Ok(crate::data::Grid::new(states));
            }
        }

        // println!("read_grid_from_folder | Returning random! \\o/");
        Ok(crate::data::Grid::random())
    }

    // Read the grid from an existing file
    fn read_state (file_path: std::path::PathBuf) -> std::io::Result<crate::data::State> {
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
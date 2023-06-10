pub(crate) struct TxtWriter {}

impl super::Writer for TxtWriter {

    // Save the history files in the specified folder
    fn write_grid(grid: &crate::data::Grid) -> std::io::Result<()> {
        super::delete_all_files_in_directory(crate::HISTORY_FOLDER)?;

        // println!("save_state_history | initializing");

        for (i, state) in grid.get_history().iter().enumerate() {
            let index = format!("{:08}", i);

            println!(
                "save_state_history | creating file [{}]: {:?}",
                index, std::path::PathBuf::from(&crate::HISTORY_FOLDER).join(format!("state_{}.txt", index))
            );

            TxtWriter::write_state(
                state,
                std::path::PathBuf::from(&crate::HISTORY_FOLDER).join(format!("state_{}.txt", index))
            )?;
        }

        // println!("save_state_history | {} files saved", state_strings.len());

        Ok(())
    }

    // Save the grid state to a file
    fn write_state(state: &crate::data::State, file_path: std::path::PathBuf) -> std::io::Result<()> {
        let mut state_string = String::new();

        for row in state.iter() {
            let row_string = row.iter().map(|cell| cell.to_string()).collect::<Vec<_>>().join(",");
            state_string.push_str(&row_string);
            state_string.push('\n');
        }

        let mut file = std::fs::File::create(&file_path)?;
        std::io::Write::write_all(&mut file, state_string.as_bytes())
    }


}
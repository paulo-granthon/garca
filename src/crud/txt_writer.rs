pub(crate) struct TxtWriter {}

impl super::Writer for TxtWriter {

    // Save the history files in the specified folder
    fn write_grid(&self, grid: Vec<crate::crud::Render>) -> std::io::Result<()> {
        for (i, state) in grid.iter().enumerate() {
            let index = format!("{:08}", i);
            let mut file = std::fs::File::create(
                std::path::PathBuf::from(&crate::HISTORY_FOLDER).join(format!("state_{}.txt", index))
            )?;
            match state {
                super::Render::TxtState(data) => std::io::Write::write_all(
                    &mut file,
                    data.as_bytes()
                )?,
                super::Render::PngState(_) => return Err(state.error("TxtWriter", "TxtState")),
            }
        }
        Ok(())
    }
}

impl super::Renderer for TxtWriter {

    // Save the grid state to a file
    fn gen_state(&self, state: &crate::data::State) -> std::io::Result<crate::crud::Render> {
        let mut state_string = String::new();
        for row in state.iter() {
            let row_string = row.iter().map(|cell| cell.to_string()).collect::<Vec<_>>().join(",");
            state_string.push_str(&row_string);
            state_string.push('\n');
        }
        Ok(super::Render::TxtState(state_string))
    }
}
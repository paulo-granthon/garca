pub(crate) trait Reader {
    fn read_grid (file_path: &str) -> std::io::Result<crate::data::Grid>;
    fn read_state (file_path: std::path::PathBuf) -> std::io::Result<crate::data::State>;    
}

pub(crate) trait Writer {
    fn write_grid (grid: &crate::data::Grid) -> Result<(), std::io::Error>;
    fn write_state (state: &crate::data::State, file_path: std::path::PathBuf) -> Result<(), std::io::Error>;
}

pub(crate) trait Renderer <T> {
    fn render_grid (grid: &crate::data::Grid) -> Result<(), std::io::Error>;
    fn gen_state (state: &crate::data::State) -> Result<T, std::io::Error>;
}

pub(crate) fn delete_all_files_in_directory(dir_path: &str) -> std::io::Result<()> {
    // Read the directory
    let dir_entries = std::fs::read_dir(dir_path)?;

    // Iterate over the directory entries
    for entry in dir_entries {
        if let Ok(entry) = entry {
            // Check if the entry is a file
            if entry.file_type()?.is_file() {
                // Delete the file
                std::fs::remove_file(entry.path())?;
            }
        }
    }

    Ok(())
}

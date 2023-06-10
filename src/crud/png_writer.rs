use image::{
    ImageBuffer,
    Rgb
};

pub(crate) struct PngWriter {}

impl super::Writer for PngWriter {
    fn write_grid (&self, grid: &crate::data::Grid) -> std::io::Result<()> {
        super::delete_all_files_in_directory(crate::HISTORY_FOLDER)?;

        for (i, state) in grid.get_history().iter().enumerate() {
            let index = format!("{:08}", i);
            let file_path = std::path::PathBuf::from(&crate::HISTORY_FOLDER).join(format!("state_{}.png", index));
            self.write_state(state, file_path)?;
        }

        Ok(())
    }

    fn write_state (&self, state: &crate::data::State, file_path: std::path::PathBuf) -> std::io::Result<()> {
        let width = crate::GRID_WIDTH as u32;
        let height = crate::GRID_HEIGHT as u32;
        let mut image_buffer = ImageBuffer::<Rgb<u8>, _>::new(width, height);

        for (y, row) in state.iter().enumerate().take(height as usize) {
            for (x, &cell) in row.iter().enumerate().take(width as usize) {
                let color = if cell == 1 { Rgb([0, 0, 0]) } else { Rgb([255, 255, 255]) };
                image_buffer.put_pixel(x as u32, y as u32, color);
            }
        }

        let result = image_buffer.save(file_path);
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(std::io::Error::new(std::io::ErrorKind::Other, error.to_string())),
        }
    }
}
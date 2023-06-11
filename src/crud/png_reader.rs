pub(crate) struct PngReader {}

impl super::Reader for PngReader {
    fn read_grid (&self, file_path: &str) -> std::io::Result<crate::data::Grid> {
        if std::path::Path::new(file_path).exists() {
            let mut states: Vec<crate::data::State> = vec![];

            let mut files = std::fs::read_dir(crate::HISTORY_FOLDER)?
                .map(|entry| entry.unwrap())
                .collect::<Vec<std::fs::DirEntry>>();
            files.sort_by(|a, b| {
                let a_name = a.file_name();
                let b_name = b.file_name();
                a_name.cmp(&b_name)
            });

            for file in files {
                let state = self.read_state(file.path())?;
                states.push(state);
            }

            if !states.is_empty() {
                return Ok(crate::data::Grid::new(states));
            }
        }

        Ok(crate::data::Grid::random())
    }

    fn read_state (&self, file_path: std::path::PathBuf) -> std::io::Result<crate::data::State> {
        match image::open(file_path) {
            Ok(image_buffer) => {
                let image_buffer = image_buffer.into_rgba8();

                let width = crate::GRID_WIDTH as u32;
                let height = crate::GRID_HEIGHT as u32;
                let resized_image = image::imageops::resize(
                    &image_buffer,
                    width,
                    height,
                    image::imageops::FilterType::Nearest
                );

                let mut state = crate::data::State::default();

                for (y, row) in resized_image.rows().enumerate().take(height as usize) {
                    for (x, pixel) in row.enumerate().take(width as usize) {
                        let rgba = pixel.0;
                        let grayscale = (rgba[0] as u32 + rgba[1] as u32 + rgba[2] as u32) / 3;
                        let cell = if grayscale >= 128 { 1 } else { 0 };
                        state[y][x] = cell;
                    }
                }

                Ok(state)

            }
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported image format")),
        }
    }
}

use image::{
    ImageBuffer,
    Rgb
};

pub(crate) struct PngWriter {}

impl super::Renderer for PngWriter {
    fn gen_state (&self, state: &crate::data::State) -> Result<super::Render, std::io::Error> {
        let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(
            crate::GRID_WIDTH as u32,
            crate::GRID_HEIGHT as u32
        );

        for (y, row) in state.iter().enumerate().take(crate::GRID_HEIGHT) {
            for (x, &cell) in row.iter().enumerate().take(crate::GRID_WIDTH) {
                let color = if cell == 1 { Rgb([255, 255, 255]) } else { Rgb([0, 0, 0]) };
                image_buffer.put_pixel(x as u32, y as u32, color);
            }
        }

        Ok(super::Render::PngState(image_buffer))
    }
}

impl super::Writer for PngWriter {
    fn write_grid (&self, states: Vec<super::Render>) -> std::io::Result<()> {
        for (i, state) in states.iter().enumerate() {
            if let Err(error) = match state {
                super::Render::PngState(data) => data,
                super::Render::TxtState(_) => return Err(state.error("PngWriter", "PngState")),
            }.save(
                std::path::PathBuf::from(&crate::HISTORY_FOLDER).join(format!(
                    "state_{}.png",
                    format!("{:08}", i))
                )
            ) {
                dbg!(std::io::Error::new(std::io::ErrorKind::InvalidData, error.to_string()));
                panic!()
            }
        }

        Ok(())
    }
}
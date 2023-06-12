mod crud_base; pub(crate) use crud_base::*;

mod txt_reader; pub(crate) use txt_reader::TxtReader;
mod txt_writer; pub(crate) use txt_writer::TxtWriter;

mod png_reader; pub(crate) use png_reader::PngReader;
mod png_writer; pub(crate) use png_writer::PngWriter;

mod svg_renderer; pub(crate) use svg_renderer::SvgRenderer;

mod svg_output; pub(crate) use svg_output::SvgOutput;

#[allow(dead_code)]
pub(crate) enum Ext {
    PNG,
    TXT,
    SVG,
    // GIF,
}

pub(crate) trait ExtType {
    type Type;
}

impl Ext {
    pub fn get_reader (&self) -> Box<dyn Reader> {
        match self {
            Ext::TXT => Box::new(TxtReader{}),
            Ext::PNG => Box::new(PngReader{}),
            _=> panic!()
        }
    }

    pub fn get_writer (&self) -> Box<dyn Writer> {
        match self {
            Ext::TXT => Box::new(TxtWriter{}),
            Ext::PNG => Box::new(PngWriter{}),
            _=> panic!()
        }
    }

    pub fn get_renderer (&self) -> Box<dyn Renderer> {
        match self {
            Ext::SVG => Box::new(SvgRenderer{}),
            Ext::PNG => Box::new(PngWriter{}),
            _=> panic!()
        }
    }

    pub fn get_outputer (&self) -> Box<dyn Output> {
        match self {
            Ext::SVG => Box::new(SvgOutput{}),
            _=> panic!(),
        }
    }

}

pub enum Render {
    TxtState(String),
    PngState(image::ImageBuffer<image::Rgb<u8>, Vec<u8>>),
}

impl Render {
    fn error (&self, class: &str, expected: &str) -> std::io::Error {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{} Error: Invalid Render variant (TxtState), expected {}", class, expected)
        )
    }
}

pub(crate) struct Crud {
    reader_type: Ext,
    writer_type: Ext,
    render_type: Ext,
    output_type: Ext,
}

impl Crud {
    pub fn new (reader_type: Ext, writer_type: Ext, render_type: Ext, output_type: Ext) -> Self {
        Self {
            reader_type,
            writer_type,
            render_type,
            output_type
        }
    }

    pub fn read (&self, folder: &str) -> std::io::Result<crate::data::Grid> {
        self.reader_type.get_reader().read_grid(folder)
    }

    pub fn write (&self, grid: &crate::data::Grid) -> std::io::Result<()> {
        super::delete_all_files_in_directory(crate::HISTORY_FOLDER)?;
        self.writer_type.get_writer().write_grid(
            grid.get_history().iter().map(|state|
                self.render_type.get_renderer().gen_state(state).unwrap_or_else(|_| panic!())
            ).collect()
        )
    }

    pub fn render (&self, grid: &crate::data::Grid) -> std::io::Result<()> {
        self.output_type.get_outputer().render(
            grid.get_history().iter().map(|state|
                self.render_type.get_renderer().gen_state(state).unwrap_or_else(|_| panic!())
            ).collect()
        )
    }

}
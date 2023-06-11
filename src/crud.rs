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
}

impl Ext {
    pub fn get_reader (&self) -> Box<dyn Reader> {
        match self {
            Ext::TXT => Box::new(TxtReader{}),
            Ext::PNG => Box::new(PngReader{}),
        }
    }

    pub fn get_writer (&self) -> Box<dyn Writer> {
        match self {
            Ext::TXT => Box::new(TxtWriter{}),
            Ext::PNG => Box::new(PngWriter{}),
        }
    }
}

pub(crate) enum RenderExt {
    SVG,    
}

impl RenderExt {
    pub fn get_renderer (&self) -> Box<dyn Renderer> {
        match self {
            RenderExt::SVG => Box::new(SvgRenderer{}),
        }
    }
    pub fn get_outputer (&self) -> Box<dyn Output> {
        match self {
            RenderExt::SVG => Box::new(SvgOutput{}),
        }
    }
}
pub(crate) struct Crud {
    reader_type: Ext,
    writer_type: Ext,
    render_type: RenderExt,
    output_type: RenderExt,
}

impl Crud {
    pub fn new (reader_type: Ext, writer_type: Ext, render_type: RenderExt, output_type: RenderExt) -> Self {
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
        self.writer_type.get_writer().write_grid(grid)
    }

    pub fn render (&self, grid: &crate::data::Grid) -> std::io::Result<()> {
        self.output_type.get_outputer().render(
            grid.get_history().iter().map(|state|
                self.render_type.get_renderer().gen_state(state).unwrap_or_else(|_| panic!())
            ).collect()
        )
    }

}
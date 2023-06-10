mod crud_base; pub(crate) use crud_base::*;

mod txt_reader; pub(crate) use txt_reader::TxtReader;
mod txt_writer; pub(crate) use txt_writer::TxtWriter;

mod png_reader; pub(crate) use png_reader::PngReader;
mod png_writer; pub(crate) use png_writer::PngWriter;

mod renderer_svg; pub(crate) use renderer_svg::RendererSVG;

#[allow(dead_code)]
pub(crate) enum Ext {
    PNG,
    TXT,
}

pub(crate) enum RenderExt {
    SVG,    
}

pub(crate) struct Crud {
    reader: Ext,
    writer: Ext,
    renderer: RenderExt,
}

impl Crud {
    pub fn new (reader: Ext, writer: Ext, renderer: RenderExt) -> Self {
        Self {
            reader,
            writer,
            renderer,
        }
    }

    pub fn reader (&self) -> Box<dyn Reader> {
        match self.reader {
            Ext::TXT => Box::new(TxtReader{}),
            Ext::PNG => Box::new(PngReader{}),
        }
    }

    pub fn writer (&self) -> Box<dyn Writer> {
        match self.writer {
            Ext::TXT => Box::new(TxtWriter{}),
            Ext::PNG => Box::new(PngWriter{}),
        }
    }

    pub fn renderer (&self) -> Box<dyn Renderer> {
        match self.renderer {
            RenderExt::SVG => Box::new(RendererSVG{}),
        }
    }

}
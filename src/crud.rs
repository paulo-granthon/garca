mod crud_base; pub(crate) use crud_base::*;

mod txt_reader; pub(crate) use txt_reader::TxtReader;
mod txt_writer; pub(crate) use txt_writer::TxtWriter;

mod renderer_svg; pub(crate) use renderer_svg::RendererSVG;

pub(crate) enum Ext {
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
        }
    }

    pub fn writer (&self) -> Box<dyn Writer> {
        match self.writer {
            Ext::TXT => Box::new(TxtWriter{}),
        }
    }

    pub fn renderer (&self) -> Box<dyn Renderer> {
        match self.renderer {
            RenderExt::SVG => Box::new(RendererSVG{}),
        }
    }

}
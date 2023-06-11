mod svg_output; pub(crate) use svg_output::SvgOutput;
mod svg_renderer; pub(crate) use svg_renderer::SvgRenderer;

// use log::warn;

pub(crate) enum RenderExt {
    SVG,
    // PNG,
    // GIF,
}

impl RenderExt {
    pub fn get_renderer (&self) -> Box<dyn Renderer> {
        match self {
            RenderExt::SVG => Box::new(SvgRenderer{}),
            // RenderExt::PNG => Box::new(PngRenderer{}),
            // RenderExt::GIF => { warn!("GIF extension is not supported for the state render step", ); Box::new(PngRenderer{}) },
        }
    }
    pub fn output (&self) -> Box<dyn Output> {
        match self {
            RenderExt::SVG => Box::new(SvgOutput{}),
        }
    }
}

pub(crate) trait Renderer {
    fn gen_state (&self, state: &crate::data::State) -> Result<String, std::io::Error>;
}

pub(crate) trait Output {
    fn render (&self, states: Vec<String>) -> Result<(), std::io::Error>;
}

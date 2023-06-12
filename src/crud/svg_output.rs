pub(crate) struct SvgOutput {}

impl crate::crud::Output for SvgOutput {

    fn render (&self, renders: Vec<super::Render>) -> Result<(), std::io::Error> {
        let mut frames = String::new();
    
        for (i, render) in renders.iter().enumerate() {
            frames.push_str(&format!(
                "\n\t<g opacity=\"{}\">\n\t\t<animate attributeName=\"opacity\" from=\"0\" to=\"1\" begin=\"{}ms\" dur=\"{}ms\" fill=\"freeze\" />",
                1 - std::cmp::min(i, 1),
                crate::SVG_START_DELAY_MS + (i * crate::SVG_FRAME_DURATION_MS),
                crate::SVG_FRAME_DURATION_MS
            ));
            if i < renders.len() - 1 {
                frames.push_str(&format!(
                    "\n\t\t<animate attributeName=\"opacity\" from=\"1\" to=\"0\" begin=\"{}ms\" dur=\"{}ms\" fill=\"freeze\" />",
                    crate::SVG_START_DELAY_MS + ((i + 1) * crate::SVG_FRAME_DURATION_MS),
                    crate::SVG_FRAME_DURATION_MS
                ));    
            }
            frames.push_str(
                &format!("{}\n\t</g>",
                match render {
                    super::Render::TxtState(data) => data.to_owned(),
                    super::Render::PngState(data) => crate::util::img_to_svg::image_to_svg(&data)
                }
            ));
        }
    
        let svg_string = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">{}\n</svg>",
            crate::GRID_WIDTH * crate::SVG_CELL_SCALE,
            crate::GRID_HEIGHT * crate::SVG_CELL_SCALE,
            frames
        );
    
        std::fs::write("main.svg", svg_string)?;
    
        Ok(())
    }    
}
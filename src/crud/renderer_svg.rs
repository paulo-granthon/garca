pub(crate) struct RendererSVG {}

impl super::Renderer for RendererSVG {

    fn render_grid (&self, grid: &crate::data::Grid) -> Result<(), std::io::Error> {
        let mut frames = String::new();
    
        for (i, state) in grid.get_history().iter().enumerate() {
            frames.push_str(&format!(
                "\n\t<g opacity=\"{}\">\n\t\t<animate attributeName=\"opacity\" from=\"0\" to=\"1\" begin=\"{}ms\" dur=\"{}ms\" fill=\"freeze\" />",
                1 - std::cmp::min(i, 1),
                crate::SVG_START_DELAY_MS + (i * crate::SVG_FRAME_DURATION_MS),
                crate::SVG_FRAME_DURATION_MS
            ));
            if i < grid.get_history().len() - 1 {
                frames.push_str(&format!(
                    "\n\t\t<animate attributeName=\"opacity\" from=\"1\" to=\"0\" begin=\"{}ms\" dur=\"{}ms\" fill=\"freeze\" />",
                    crate::SVG_START_DELAY_MS + ((i + 1) * crate::SVG_FRAME_DURATION_MS),
                    crate::SVG_FRAME_DURATION_MS
                ));    
            }
            frames.push_str(&format!("{}\n\t</g>", self.gen_state(&state)?));
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
    
    
    // Generate an SVG string representation of the grid state
    fn gen_state (&self, state: &crate::data::State) -> Result<String, std::io::Error> {
        let mut svg_string = String::new();

        svg_string.push_str("\n\t\t<g>");

        for (i, row) in state.get_cells().iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let x = j * crate::SVG_CELL_SCALE;
                let y = i * crate::SVG_CELL_SCALE;

                // SVG rectangle representing the cell
                svg_string.push_str(&format!(
                    "\n\t\t\t<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" />",
                    x,
                    y,
                    crate::SVG_CELL_SCALE,
                    crate::SVG_CELL_SCALE,
                    if *cell == 1 { "white" } else { "black" }
                ));
            }
        }

        // SVG footer
        svg_string.push_str("\n\t\t</g>");

        Ok(svg_string)
    }
    
}


pub(crate) struct SvgRenderer {}

impl super::Renderer for SvgRenderer {

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
use base64::engine::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD as BASE64;

#[allow(deprecated)]
pub(crate) fn image_to_svg(image_buffer: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> String {
    let (width, height) = image_buffer.dimensions();

    // Start building the SVG string
    let mut svg = String::new();

    // Add the image data as a base64-encoded PNG
    let mut png_data = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_data);

    // Write the image data to the encoder
    encoder.encode(&image_buffer, width, height, image::ColorType::Rgb8).expect("Failed to encode PNG");

    svg.push_str("\n\t\t<image xlink:href=\"data:image/png;charset=utf-8;base64,");
    svg.push_str(BASE64.encode(png_data).as_str());
    svg.push_str("\"");
    svg.push_str(format!(" width=\"{}\" height=\"{}\"", width, height).as_str());
    svg.push_str("/>\n");

    svg
}

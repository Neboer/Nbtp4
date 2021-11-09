use std::io::Cursor;
use png::EncodingError;
use crate::definition::SIZE;

fn _encode_to_png(data: &Vec<u8>) -> Result<Vec<u8>, EncodingError> {
    let mut result_image = Vec::new();
    let mut w = Cursor::new(&mut result_image);
    let mut encoder = png::Encoder::new(w, SIZE, SIZE);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
                                                                    (0.31270, 0.32900),
                                                                    (0.64000, 0.33000),
                                                                    (0.30000, 0.60000),
                                                                    (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(data.as_ref())?;
    writer.finish()?;
    Ok(result_image)
}

pub trait EncodeToPng {
    fn encode
}

impl EncodeToPng for Chunk {

}
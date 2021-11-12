use crate::advanced_read::advanced_read_exact;
use crate::codec::to_rectangle::find_rectangle;
use crate::definition::{ChunkMeta, ImageSetting, ImgChunk, RawChunk, RetSize, SIZE};
use png::EncodingError;
use std::cmp::{max, min};
use std::io::{Cursor, Error, Read};

fn encode_to_png(data: &Vec<u8>, size: RetSize, alpha: bool) -> Result<Vec<u8>, EncodingError> {
    let unit_size = (alpha as u32) * 1 + 3;
    assert_eq!(
        size.t_size(unit_size as u32) as usize,
        data.len(),
        "mismatched image size!"
    );
    let color = if alpha {
        png::ColorType::Rgba
    } else {
        png::ColorType::Rgb
    };
    let mut result_image = Vec::new();
    let mut w = Cursor::new(&mut result_image);
    let mut encoder = png::Encoder::new(w, size.width, size.height);
    encoder.set_color(color);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
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

pub struct ImgStreamEncoder<R: Read> {
    head_pos: u32,
    reader: R,
    image_setting: ImageSetting,
    eof: bool,
}

impl<R: Read> ImgStreamEncoder<R> {
    pub fn encode_from_stream(&mut self) -> Result<ImgChunk, Error> {
        // ImgChunk 的chunklength如果为0，说明转换已经结束。可以查看Encoder的eof属性确认这一点
        // 按照max_wh读取，如果不够，根据min_wh读取。
        let unit_size = (self.image_setting.alpha as u32) * 1 + 3;
        let max_read_size = self.image_setting.max_size.t_size(unit_size);
        let mut result_chunk = ImgChunk {
            data: vec![],
            meta: ChunkMeta {
                first_pos: self.head_pos,
                img_size: self.image_setting.max_size,
                alpha: self.image_setting.alpha,
                chunk_size: 0,
            },
        };

        let mut buffer: Vec<u8> = vec![0u8; max_read_size as usize];

        // 根据实际读取的内容对buffer和result_chunk进行处理修改。
        match advanced_read_exact(&mut self.reader, &mut buffer) {
            // 数据都读进了buffer
            Ok(read_size) => {
                let read_count = read_size as u32;
                result_chunk.meta.chunk_size = read_count;
                if read_count < max_read_size {
                    // 发现已经读到文件末尾
                    self.eof = true;
                    let minimal_size = self.image_setting.min_size.t_size(unit_size);
                    if read_count < minimal_size {
                        // 读取长度小于最小图片大小，需要补充。
                        buffer.extend_from_slice(&vec![0u8; (minimal_size - read_count) as usize]);
                        result_chunk.meta.img_size = self.image_setting.min_size;
                        result_chunk.meta.chunk_size = read_count;
                        result_chunk.data = encode_to_png(
                            &result_chunk.data,
                            result_chunk.meta.img_size,
                            result_chunk.meta.alpha,
                        )?;
                    } else if read_count < max_read_size {
                        // 读取长度小于最大大小，但是大于最小大小。
                        result_chunk.meta.chunk_size = read_count;
                        // 找到一个距离read_count最近的矩形之前，先把read_count补成可以被自身色深整除的大小。
                        let target_size = find_rectangle(
                            (read_count as f32 / unit_size as f32).ceil() as u32,
                            self.image_setting.max_size,
                            self.image_setting.min_size,
                        );
                        result_chunk.meta.img_size = target_size;
                        buffer.extend_from_slice(&vec![
                            0u8;
                            (target_size.t_size(unit_size) - read_count)
                                as usize
                        ])
                    }
                }
            }
            Err(e) => return Err(e),
        }
        result_chunk.data =
            encode_to_png(&buffer, result_chunk.meta.img_size, result_chunk.meta.alpha)?;
        Ok(result_chunk)
    }
}

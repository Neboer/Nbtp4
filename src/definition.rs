pub const SIZE: u32 = 1000;

#[derive(Copy, Clone, Debug)]
pub struct RetSize {
    pub width: u32,
    pub height: u32
}

pub struct ChunkMeta {
    pub first_pos: u32,
    pub img_size: RetSize,
    pub alpha: bool,
    pub chunk_size: u32
}

pub struct RawChunk {
    data: Vec<u8>,
    meta: ChunkMeta,
}

pub struct ImgChunk {
    pub data: Vec<u8>,
    pub meta: ChunkMeta,
}

pub struct NetChunk {
    data: Vec<u8>,
    meta: ChunkMeta,
}

pub struct KeyChunk {
    data: Vec<u8>,
    meta: ChunkMeta,
}

// 图床对图片的要求，设为0表示无限制，min字段必须设置。
pub struct ImageSetting {
    pub max_size: RetSize,
    pub min_size: RetSize,
    pub alpha: bool,
}

impl RetSize {
    pub fn area(&self) -> u32 {
        (self.height * self.width) as u32
    }

    pub fn t_size(&self, byte_per_pixel: u32) -> u32 {
        (self.height * self.width * byte_per_pixel as u32) as u32
    }
}
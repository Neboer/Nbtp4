pub const SIZE: u32 = 1000;

struct ChunkMeta {
    data: Vec<u8>,
    first_pos: usize
}

enum Chunk {
    Raw(ChunkMeta),
    Img(ChunkMeta),
    Pac(ChunkMeta),
    Key(ChunkMeta)
}

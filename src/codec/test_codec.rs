use crate::codec::encoder::encode_chunk_to_png;
use crate::definition::SIZE;

#[test]
fn test_encoder() {
    let mut dest_png = Vec::new();
    let mut src_data: Vec<u8> = Vec::with_capacity((3 * SIZE * SIZE) as usize);
    for i in 0..(3 * SIZE * SIZE) {
        src_data.push((i % 255) as u8)
    }
    encode_chunk_to_png(&src_data, &mut dest_png).unwrap();
}
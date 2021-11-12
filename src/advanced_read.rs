use std::io::{Error, ErrorKind, Read};

pub fn advanced_read_exact<R: Read>(reader: &mut R, mut buf: &mut [u8]) -> Result<usize, Error> {
    let mut total: usize = 0;
    while !buf.is_empty() {
        match reader.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                total += n;
                let tmp = buf;
                buf = &mut tmp[n..];
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(total),
            Err(e) => return Err(e),
        }
    }
    Ok(total)
}


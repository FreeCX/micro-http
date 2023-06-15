use std::io::Read;

use crate::error::FrameworkError;

// https://doc.rust-lang.org/src/std/io/mod.rs.html#1924
pub fn until_crlf<R: Read>(r: &mut R) -> Result<String, FrameworkError> {
    let mut buf = Vec::new();

    for b in r.bytes() {
        let b = b?;
        buf.push(b);
        if buf.ends_with(b"\r\n\r\n") {
            break;
        }
    }

    String::from_utf8(buf).map_err(FrameworkError::from)
}

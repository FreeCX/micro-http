use std::io::Read;

pub fn read_until_crlf<R: Read>(r: &mut R) -> Option<String> {
    let mut buf = Vec::new();

    for b in r.bytes() {
        let b = b.ok()?;
        buf.push(b);
        if buf.ends_with(b"\r\n\r\n") {
            break;
        }
    }

    String::from_utf8(buf).ok()
}

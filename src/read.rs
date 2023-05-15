use std::io::Read;

pub fn read_all<R: Read>(r: &mut R) -> Option<String> {
    let mut probe = [0u8; 32];
    let mut buf = Vec::new();

    loop {
        match r.read(&mut probe) {
            Ok(0) => break,
            Ok(n) => {
                buf.extend_from_slice(&probe[..n]);
                if probe.ends_with(b"\r\n\r\n") {
                    break;
                }
                continue;
            }
            Err(_) => break,
        }
    }

    String::from_utf8(buf).ok()
}

pub fn read_until_crlf<R: Read>(r: &mut R) -> Option<String> {
    let mut buf = Vec::new();

    for b in r.bytes() {
        let b = b.unwrap();
        buf.push(b);
        if buf.ends_with(b"\r\n\r\n") {
            break;
        }
    }

    String::from_utf8(buf).ok()
}

use std::io::{self, Read, Write};

const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn encode_base64(input: &[u8]) -> String {
    let mut result = Vec::with_capacity((input.len() + 2) / 3 * 4);

    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;

        let combined = (b0 << 16) | (b1 << 8) | b2;

        result.push(BASE64_TABLE[((combined >> 18) & 0x3F) as usize]);
        result.push(BASE64_TABLE[((combined >> 12) & 0x3F) as usize]);

        if chunk.len() > 1 {
            result.push(BASE64_TABLE[((combined >> 6) & 0x3F) as usize]);
        } else {
            result.push(b'=');
        }

        if chunk.len() > 2 {
            result.push(BASE64_TABLE[(combined & 0x3F) as usize]);
        } else {
            result.push(b'=');
        }
    }

    String::from_utf8(result).unwrap()
}

fn main() -> io::Result<()> {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input)?;

    let encoded = encode_base64(&input);
    io::stdout().write_all(encoded.as_bytes())?;

    Ok(())
}

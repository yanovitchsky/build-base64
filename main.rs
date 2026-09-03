use std::io::{self, BufRead};

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const CHUNK_SIZE: usize = 3;
const CHUNK_SIZE_DECODE: usize = 4;

fn encode(input: &[u8]) -> String {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);

    for chunk in input.chunks(CHUNK_SIZE) {
        let mut b = (chunk[0] as u32) << 16;
        if chunk.len() > 1 {
            b |= (chunk[1] as u32) << 8;
        }
        if chunk.len() > 2 {
            b |= chunk[2] as u32;
        }

        let idx1 = ((b >> 18) & 0x3F) as usize;
        let idx2 = ((b >> 12) & 0x3F) as usize;
        let idx3 = ((b >> 6) & 0x3F) as usize;
        let idx4 = (b & 0x3F) as usize;

        out.push(BASE64_CHARS[idx1] as char);
        out.push(BASE64_CHARS[idx2] as char);

        if chunk.len() > 2 {
            out.push(BASE64_CHARS[idx3] as char);
            out.push(BASE64_CHARS[idx4] as char);
        } else if chunk.len() > 1 {
            out.push(BASE64_CHARS[idx3] as char);
            out.push('=');
        } else {
            out.push('=');
            out.push('=');
        }
    }
    out
}

fn b64_val(c: u8) -> Option<usize> {
    BASE64_CHARS.iter().position(|&x| x == c)
}

fn decode(input: &str) -> String {
    let mut out = Vec::with_capacity(input.len() / 4 * 3);

    for chunk in input.as_bytes().chunks(CHUNK_SIZE_DECODE) {
        let Some(idx1) = b64_val(chunk[0]) else {
            return String::new();
        };
        let Some(idx2) = b64_val(chunk[1]) else {
            return String::new();
        };

        let mut b = ((idx1 as u32) << 18) | ((idx2 as u32) << 12);
        if chunk[2] != b'=' {
            let Some(idx3) = b64_val(chunk[2]) else {
                return String::new();
            };
            b |= (idx3 as u32) << 6;
        }
        if chunk[3] != b'=' {
            let Some(idx4) = b64_val(chunk[3]) else {
                return String::new();
            };
            b |= idx4 as u32;
        }

        out.push(((b >> 16) & 0xFF) as u8);
        if chunk[2] != b'=' {
            out.push(((b >> 8) & 0xFF) as u8);
        }
        if chunk[3] != b'=' {
            out.push((b & 0xFF) as u8);
        }
    }
    String::from_utf8(out).unwrap_or_default()
}

fn command(input: &str) -> String {
    let intent = input.split_whitespace().collect::<Vec<_>>();
    if intent.is_empty() {
        return String::new();
    }
    if intent[1].is_empty() {
        return String::new();
    }

    let cmd = intent[0];
    match cmd {
        "ENCODE" => encode(&intent[1].as_bytes()),
        "DECODE" => decode(&intent[1]),
        _ => String::new(),
    }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        println!("{}", command(&l));
    }
}

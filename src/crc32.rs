use wasm_bindgen::prelude::*;

//TODO: consider a preprocessed lookup table
fn fill_crc_table(poly: u32) -> [u32; 256] {
    let mut lut: [u32; 256] = [0; 256];
    let mut crc: u32;
    for i in 0..256 {
        crc = i;
        for _j in 0..8 {
            if crc & 1 == 1 {
                crc >>= 1;
                crc ^= poly;
            } else {
                crc >>= 1;
            }
        }
        lut[i as usize] = crc;
    }
    lut
}

lazy_static! {
    static ref CRC32_LOOKUP_TABLE: [u32; 256] = fill_crc_table(0xEDB88320);
}

fn crc32_runner(lut: [u32; 256], str: &str) -> u32 {
    let mut crc32: u32 = 0xFFFFFFFF;
    let bytes = str.as_bytes();
    let length = bytes.len();
    for i in 0..length {
        let byte = bytes[i] as u32;
        let index = ((byte ^ crc32) & 0xFF) as usize;
        crc32 = lut[index] ^ ((crc32 >> 8) & 0xFFFFFF);
    }
    crc32 = !crc32;
    crc32
}

#[wasm_bindgen]
pub fn crc32(s: &str) -> u32 {
    crc32_runner(*CRC32_LOOKUP_TABLE, s)
}

#[wasm_bindgen]
pub fn crc32_be(s: &str) -> Box<[u8]> {
    let crc32 = crc32_runner(*CRC32_LOOKUP_TABLE, s);
    let vec = vec![
        ((crc32 >> 24) & 0xFF ) as u8,
        ((crc32 >> 16) & 0xFF ) as u8,
        ((crc32 >> 8) & 0xFF ) as u8,
        (crc32 & 0xFF) as u8
    ];
    return vec.into_boxed_slice()
}

#[wasm_bindgen]
pub fn crc32_le(s: &str) -> Box<[u8]> {
    let crc32 = crc32_runner(*CRC32_LOOKUP_TABLE, s);
    let vec = vec![
        (crc32 & 0xFF) as u8,
        ((crc32 >> 8) & 0xFF ) as u8,
        ((crc32 >> 16) & 0xFF ) as u8,
        ((crc32 >> 24) & 0xFF ) as u8
    ];
    return vec.into_boxed_slice()
}

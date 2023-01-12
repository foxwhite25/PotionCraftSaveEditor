use base64::{Engine as _, engine::general_purpose};

const ENCRYPTION_KEY: [u8; 16] = [97, 94, 49, 57, 117, 104, 37, 52, 55, 120, 55, 49, 101, 37, 115, 100];

pub fn decode_xor(data: &Vec<u8>) -> Vec<u8> {
    data.iter().enumerate().map(|(i, &x)| (x ^ ENCRYPTION_KEY[i % ENCRYPTION_KEY.len()])).collect()
}

pub fn decode(data: Vec<u8>) -> Vec<u8> {
    let decoded = general_purpose::STANDARD.decode(data).unwrap();
    decode_xor(&decoded)
}

pub fn encode(data: Vec<u8>) -> String {
    let encoded = decode_xor(&data);
    general_purpose::STANDARD.encode(encoded)
}
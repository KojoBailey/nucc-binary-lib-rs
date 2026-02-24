use std::sync::OnceLock;

static HASH_ARRAY: OnceLock<Vec<u32>> = OnceLock::new();

fn get_hash_array() -> &'static Vec<u32> {
    HASH_ARRAY.get_or_init(|| {
        let polynomial: u32 = 0x4C11DB7;
        let mut result = Vec::with_capacity(256);
        for i in 0u32..256 {
            let mut k: u32 = i << 24;
            for _ in 0..8 {
                if (k & 0x80000000) != 0 {
                    k = (k << 1) ^ polynomial;
                } else {
                    k = k << 1;
                }
            }
            result.push(k);
        }
        result
    })
}

pub fn hash(string: &str) -> u32 {
    let mut v2 = u32::MAX;
    for byte in string.bytes() {
        let v4 = ((v2 >> 24) as u8 ^ byte) as usize;
        v2 = (v2 << 8) ^ get_hash_array()[v4];
    }

    (!v2).to_be()
}

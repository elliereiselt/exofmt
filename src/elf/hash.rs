use std::borrow::Cow;

pub struct Hash<'a> {
    pub buckets: Cow<'a, [u32]>,
    pub chains: Cow<'a, [u32]>,
}

pub fn generate_hash(name: &str) -> u32 {
    let mut h: u32 = 0;
    let mut g: u32;

    for byte in name.as_bytes() {
        h = (h << 4) + u32::from(*byte);

        g = h & 0xf0000000;

        if g != 0 {
            h ^= g >> 24;
        }

        h &= !g;
    }

    h
}

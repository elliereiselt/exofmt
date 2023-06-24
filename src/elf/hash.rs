/*
 * Copyright 2023 Ellie Reiselt
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub struct Hash {
    pub buckets: Vec<u32>,
    pub chains: Vec<u32>,
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

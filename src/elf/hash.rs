// exofmt - binary format parser for ELF, Dex, and more.
// Copyright (C) 2023  Ellie Reiselt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
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

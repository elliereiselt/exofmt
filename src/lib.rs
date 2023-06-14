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
mod error;
pub use error::*;

pub mod leb128;
pub mod mutf8;

pub mod dex;
pub mod elf;

pub(crate) mod stringable_consts_blocks;

#[cfg(test)]
mod tests {
    #[test]
    fn size_check() {
        assert!(::std::mem::size_of::<usize>() >= 4);
    }
}

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
use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct TryItem {
    /// Start address of the block of code covered by this entry.
    /// The address is a count of 16-bit code units to the start of the first covered instruction
    pub start_address: u32,
    /// Number of 16-bit code units covered by this entry. The last code unit covered (inclusive)
    /// is `start_address + instruction_count - 1`
    pub instruction_count: u16,
    /// Index into the `handlers` list for this try item
    pub handler_index: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_item_method_annotation_size_check() {
        assert_eq!(::std::mem::size_of::<TryItem>(), 0x8);
    }
}

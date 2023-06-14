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
pub struct TypeIdItem {
    /// Index into the `string_ids` list for the descriptor string of this type. The string must conform to the syntax for [TypeDescriptor](https://source.android.com/docs/core/runtime/dex-format#typedescriptor)
    pub descriptor_index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_id_item_size_check() {
        assert_eq!(::std::mem::size_of::<TypeIdItem>(), 0x4);
    }
}

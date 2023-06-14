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
pub struct MethodAnnotation {
    pub method_index: u32,
    /// Offset from the start of the file to the list of annotations for the method.
    /// The offset should be to a location in the data section.
    pub annotations_offset: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_annotation_size_check() {
        assert_eq!(::std::mem::size_of::<MethodAnnotation>(), 0x8);
    }
}

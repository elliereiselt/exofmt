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
pub struct AnnotationsDirectoryItemHeader {
    /// Offset from the start of the file to the annotations made directly on the class, or `0` if the class has no direct annotations.
    /// The offset, if non-zero, should be to a location in the data section. The format of the data is specified by "annotation_set_item"
    pub class_annotations_offset: u32,
    /// Count of fields annotated by this item
    pub fields_size: u32,
    /// Count of methods annotated by this item
    pub annotated_methods_size: u32,
    /// Count of method parameter lists annotated by this item
    pub annotated_parameters_size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn annotations_directory_item_header_size_check() {
        assert_eq!(
            ::std::mem::size_of::<AnnotationsDirectoryItemHeader>(),
            0x10
        );
    }
}

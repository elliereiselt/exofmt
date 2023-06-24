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

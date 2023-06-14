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
use crate::dex::EncodedAnnotation;
use crate::stringable_consts_blocks::stringable_consts_block;

pub struct AnnotationItem {
    /// Intended visibility of this annotation
    pub visibility: u8,
    /// Encoded annotation contents
    pub annotation: EncodedAnnotation,
}

// Visibility constants
stringable_consts_block! {
    const stringable: u8 {
        /// Intended only to be visible at build time (e.g., during compilation of other code)
        pub VISIBILITY_BUILD = 0x00;
        /// Intended to visible at runtime
        pub VISIBILITY_RUNTIME = 0x01;
        /// Intended to visible at runtime, but only to the underlying system (and not to regular user code)
        pub VISIBILITY_SYSTEM = 0x02;
    }

    const ignore: u8 {}

    pub fn visibility_to_str(value: u8) -> &'static str {
        match value {
            _unknown => "VISIBILITY_UNKNOWN",
        }
    }
}

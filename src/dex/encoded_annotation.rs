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
use crate::dex::AnnotationElement;

pub struct EncodedAnnotation {
    /// Type of the annotation. This must be a class (not array or primitive) type
    pub type_index: u32,
    /// Elements of the annotation, represented directly in-line (not as offsets).
    /// Elements must be sorted in increasing order by `name_index`
    pub elements: Vec<AnnotationElement>,
}

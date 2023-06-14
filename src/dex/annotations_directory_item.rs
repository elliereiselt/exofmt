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
use crate::dex::AnnotationItem;
use crate::dex::FieldAnnotation;
use crate::dex::MethodAnnotation;
use crate::dex::ParameterAnnotation;

pub struct AnnotationsDirectoryItem {
    /// Annotations made directly on the class
    pub class_annotations: Vec<AnnotationItem>,
    /// List of associated field annotations
    pub field_annotations: Vec<FieldAnnotation>,
    /// List of associated method annotations
    pub method_annotations: Vec<MethodAnnotation>,
    /// List of associated method parameter annotations
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

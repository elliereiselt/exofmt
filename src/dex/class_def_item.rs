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
use crate::dex::AnnotationsDirectoryItem;
use crate::dex::ClassDataItem;
use crate::dex::EncodedValue;
use crate::dex::TypeItem;

pub struct ClassDefItem {
    /// Index into the `type_ids` list for this class. This must be a class type, and not an array or primitive type
    pub class_index: u32,
    /// Access flags for the class (public, final, etc.)
    pub access_flags: u32,
    /// Index into the `type_ids` list for the superclass, or the constant value `NO_INDEX` if this class has no superclass
    /// (i.e., it is a root class such as Object). If present, this must be a class type, and not an array or primitive type
    pub superclass_index: u32,
    /// List of interfaces this class implements. Each of the elements of the list must be a class type
    /// (not an array or primitive type), and there must not be any duplicates
    pub interfaces: Vec<TypeItem>,
    /// Index into the `string_ids` list for the name of the file containing the original source for (at least most of) this
    /// class, or the special value `NO_INDEX` to represent a lack of this information. The `debug_info_item` of any given
    /// method may override this source file, but the expectation is that most classes will only come from one source file
    pub source_file_index: u32,
    /// The annotations structure for this class
    pub annotations: Option<AnnotationsDirectoryItem>,
    /// The associated class data for this item
    pub class_data: Option<ClassDataItem>,
    /// List of initial values for static fields. The elements correspond to the static fields in the same order as declared in the corresponding `field_list`
    ///
    /// NOTE: If this is empty but the corresponding `field_list` is not, it will be cause all static fields to be initialized with `0` or `null`
    pub static_values: Vec<EncodedValue>,
}

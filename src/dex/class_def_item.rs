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

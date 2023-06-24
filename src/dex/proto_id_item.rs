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

use crate::dex::TypeItem;

pub struct ProtoIdItem {
    /// Index into the `string_ids` list for the short-form descriptor string of this prototype.
    /// The string must conform to the syntax for [ShortyDescriptor](https://source.android.com/docs/core/runtime/dex-format#shortydescriptor)
    /// and must correspond to the return type and parameters of this item
    pub shorty_index: u32,
    /// Index into the `type_ids` list for the return type of this prototype
    pub return_type_index: u32,
    /// List of `TypeItem` values for the parameter types of this prototype
    pub parameters: Vec<TypeItem>,
}

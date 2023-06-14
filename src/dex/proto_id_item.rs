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

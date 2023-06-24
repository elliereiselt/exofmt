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
pub struct MethodIdItem {
    /// Index into the `type_ids` list for the definer of this method. This must be a class or array type, and not a primitive type
    pub class_index: u16,
    /// Index into the `proto_ids` list for the prototype of this method
    pub proto_index: u16,
    /// Index into the `string_ids` list for the name of this method. The string must conform to the syntax for [MemberName](https://source.android.com/docs/core/runtime/dex-format#membername)
    pub name_index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_id_item_size_check() {
        assert_eq!(::std::mem::size_of::<MethodIdItem>(), 0x8);
    }
}

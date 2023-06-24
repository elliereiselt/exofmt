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

use crate::stringable_consts_blocks::stringable_consts_block;
use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct MethodHandleItem {
    pub method_handle_type: u16,
    pub _unused0: u16,
    pub field_or_method_id: u16,
    pub _unused1: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_handle_item_size_check() {
        assert_eq!(::std::mem::size_of::<MethodHandleItem>(), 0x8);
    }
}

stringable_consts_block! {
    const stringable: u16 {
        /// Method handle is a static field setter (accessor)
        pub METHOD_HANDLE_TYPE_STATIC_PUT = 0x00;
        /// Method handle is a static field getter (accessor)
        pub METHOD_HANDLE_TYPE_STATIC_GET = 0x01;
        /// Method handle is an instance field setter (accessor)
        pub METHOD_HANDLE_TYPE_INSTANCE_PUT = 0x02;
        /// Method handle is an instance field getter (accessor)
        pub METHOD_HANDLE_TYPE_INSTANCE_GET = 0x03;
        /// Method handle is a static method invoker
        pub METHOD_HANDLE_TYPE_INVOKE_STATIC = 0x04;
        /// Method handle is an instance method invoker
        pub METHOD_HANDLE_TYPE_INVOKE_INSTANCE = 0x05;
        /// Method handle is a constructor method invoker
        pub METHOD_HANDLE_TYPE_INVOKE_CONSTRUCTOR = 0x06;
        /// Method handle is a direct method invoker
        pub METHOD_HANDLE_TYPE_INVOKE_DIRECT = 0x07;
        /// Method handle is an interface method invoker
        pub METHOD_HANDLE_TYPE_INVOKE_INTERFACE = 0x08;
    }

    const ignore: u16 {}

    pub fn method_handle_type_to_str(value: u16) -> &'static str {
        match value {
            _unknown => "METHOD_HANDLE_TYPE_UNKNOWN"
        }
    }
}

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

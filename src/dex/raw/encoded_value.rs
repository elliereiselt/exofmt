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

stringable_consts_block! {
    const stringable: u8 {
        pub VALUE_BYTE = 0x0;
        pub VALUE_SHORT = 0x02;
        pub VALUE_CHAR = 0x03;
        pub VALUE_INT = 0x04;
        pub VALUE_LONG = 0x06;
        pub VALUE_FLOAT = 0x10;
        pub VALUE_DOUBLE = 0x11;
        pub VALUE_METHOD_TYPE = 0x15;
        pub VALUE_METHOD_HANDLE = 0x16;
        pub VALUE_STRING = 0x17;
        pub VALUE_TYPE = 0x18;
        pub VALUE_FIELD = 0x19;
        pub VALUE_METHOD = 0x1a;
        pub VALUE_ENUM = 0x1b;
        pub VALUE_ARRAY = 0x1c;
        pub VALUE_ANNOTATION = 0x1d;
        pub VALUE_NULL = 0x1e;
        pub VALUE_BOOLEAN = 0x1f;
    }

    const ignore: u8 {}

    pub fn value_to_str(value: u8) -> &'static str {
        match value {
            _unknown => "VALUE_UNKNOWN",
        }
    }
}

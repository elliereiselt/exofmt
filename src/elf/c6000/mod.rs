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

use crate::stringable_consts_blocks::option_stringable_consts_block;

// OS ABI types
option_stringable_consts_block! {
    const stringable: u8 {
        /// Bare-metal TMS320C6000
        pub OSABI_C6000_ELFABI = 64;
        /// Linux TMS320C6000
        pub OSABI_C6000_LINUX = 65;
    }

    const ignore: u8 {}

    pub fn osabi_to_str(value: u8) -> Option<&'static str>;
}

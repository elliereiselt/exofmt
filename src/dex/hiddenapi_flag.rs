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

pub type HiddenApiRestriction = u32;

stringable_consts_block! {
    const stringable: u32 {
        pub HIDDENAPI_FLAG_WHITELIST = 0x00;
        pub HIDDENAPI_FLAG_GREYLIST = 0x01;
        pub HIDDENAPI_FLAG_BLACKLIST = 0x02;
        pub HIDDENAPI_FLAG_GREYLIST_MAX_O = 0x03;
        pub HIDDENAPI_FLAG_GREYLIST_MAX_P = 0x04;
        pub HIDDENAPI_FLAG_GREYLIST_MAX_Q = 0x05;
        pub HIDDENAPI_FLAG_GREYLIST_MAX_R = 0x06;
    }

    const ignore: u32 {}

    pub fn hiddenapi_flag_to_str(value: u32) -> &'static str {
        match value {
            _unknown => "HIDDENAPI_FLAG_UNKNOWN"
        }
    }
}

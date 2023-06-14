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

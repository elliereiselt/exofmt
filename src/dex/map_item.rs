use crate::stringable_consts_blocks::stringable_consts_block;
use scroll_derive::{IOread, IOwrite, Pread, Pwrite, SizeWith};

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Pread, Pwrite, IOread, IOwrite, SizeWith)]
pub struct MapItem {
    /// Type of the items found at `offset`
    pub type_code: u16,
    _unused: u16,
    /// Count of the number of items found at `offset`
    pub size: u32,
    /// Offset from the start of the file to the items
    pub offset: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_item_size_check() {
        assert_eq!(::std::mem::size_of::<MapItem>(), 0xc);
    }
}

stringable_consts_block! {
    const stringable: u16 {
        /// Type code for a header item
        pub TYPE_HEADER_ITEM = 0x0000;
        /// Type code for a string id item
        pub TYPE_STRING_ID_ITEM = 0x0001;
        /// Type code for a type id item
        pub TYPE_TYPE_ID_ITEM = 0x0002;
        /// Type code for a proto id item
        pub TYPE_PROTO_ID_ITEM = 0x0003;
        /// Type code for a field id item
        pub TYPE_FIELD_ID_ITEM = 0x0004;
        /// Type code for a method id item
        pub TYPE_METHOD_ID_ITEM = 0x0005;
        /// Type code for a class def item
        pub TYPE_CLASS_DEF_ITEM = 0x0006;
        /// Type code for a call site id item
        pub TYPE_CALL_SITE_ID_ITEM = 0x0007;
        /// Type code for a method handle item
        pub TYPE_METHOD_HANDLE_ITEM = 0x0008;
        /// Type code for a map list
        pub TYPE_MAP_LIST = 0x1000;
        /// Type code for a type list
        pub TYPE_TYPE_LIST = 0x1001;
        /// Type code for an annotation set ref list
        pub TYPE_ANNOTATION_SET_REF_LIST = 0x1002;
        /// Type code for an annotation set item
        pub TYPE_ANNOTATION_SET_ITEM = 0x1003;
        /// Type code for a class data item
        pub TYPE_CLASS_DATA_ITEM = 0x2000;
        /// Type code for a code item
        pub TYPE_CODE_ITEM = 0x2001;
        /// Type code for a string data item
        pub TYPE_STRING_DATA_ITEM = 0x2002;
        /// Type code for a debug info item
        pub TYPE_DEBUG_INFO_ITEM = 0x2003;
        /// Type code for an annotation item
        pub TYPE_ANNOTATION_ITEM = 0x2004;
        /// Type code for an encoded array item
        pub TYPE_ENCODED_ARRAY_ITEM = 0x2005;
        /// Type code for an annotations directory item
        pub TYPE_ANNOTATIONS_DIRECTORY_ITEM = 0x2006;
        /// Type code for a hiddenapi class data item
        pub TYPE_HIDDENAPI_CLASS_DATA_ITEM = 0xF000;
    }

    const ignore: u16 {}

    pub fn type_to_str(value: u16) -> &'static str {
        match value {
            _unknown => "TYPE_UNKNOWN"
        }
    }
}
